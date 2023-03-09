use futures::join;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rust_parallel_operations::{
    di::handler_di::{AppClient, AppInitialisation},
    queries::{operation1::Operation1, operation2::Operation2},
};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let operation1 = Operation1::builder()
        .table_name("table_name")
        .client(dynamodb_client.clone())
        .build();

    let operation2 = Operation2::builder()
        .table_name("table_name")
        .client(dynamodb_client.clone())
        .build();

    let app_client = AppClient::builder()
        .operation1(operation1)
        .operation2(operation2)
        .build();

    run(service_fn(|event: LambdaEvent<Value>| {
        function_handler(&app_client, event)
    }))
    .await
}

pub async fn function_handler(
    app_client: &dyn  AppInitialisation,
    event: LambdaEvent<Value>,
) -> Result<(), Error> {
    println!("{event:?}");

    // sequentially - unit test will pass
    // let result1 = app_client.operation1("something").await?;
    // let result2 = app_client.operation2("something").await?;

    //futures::join! runs things “on the same thread” conceptually:
    // join!(a, b) is similar to (a.await, b.await)
    // so it doesn’t need to worry about the multi-threaded data-sharing aspects of Rust.
    let (result1, result2) = join!(
        app_client.operation1("something"),
        app_client.operation2("something"),
    );
    println!("{result1:?}");
    println!("{result2:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::mock;
    use rust_parallel_operations::error::ApplicationError;

    #[tokio::test]
    async fn invalidate_key() -> Result<(), ApplicationError> {
        // ARRANGE
        let json = r#"{"something": "something"}"#;
        let request: Value = serde_json::from_str(json).unwrap();
        let context = lambda_runtime::Context::default();
        let event = LambdaEvent::new(request, context);

        mock! {
            pub AppClient {}

            #[async_trait]
            impl AppInitialisation for AppClient {
                async fn operation1(&self, something: &str) -> Result<String, ApplicationError>;
                async fn operation2(&self, something: &str) -> Result<Option<String>, ApplicationError>;
            }
        }

        let mut mock = MockAppClient::new();
        mock.expect_operation1()
            .times(1)
            .returning(move |_| Ok("ciao".to_string()));
        mock.expect_operation2()
            .times(1)
            .returning(move |_| Ok(Some("ciao".to_string())));

        // ACT
        let result = function_handler(&mock, event).await;

        // ASSERT
        assert_eq!(result.is_ok(), true);
        Ok(())
    }
}
