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

    // But from a Rust compiler point of view, it doesn’t actually think that app_client has a 'static lifetime. And tokio::spawn requires things to be static.
    // So one option is: tell the compiler that app_client is 'static. And we can do that by leaking its memory (so it never gets cleaned up).
    let client: &'static AppClient = Box::leak(Box::new(app_client));
    run(service_fn(|event: LambdaEvent<Value>| {
        function_handler(client, event)
    }))
    .await
}

pub async fn function_handler(
    app_client: &'static dyn AppInitialisation,
    event: LambdaEvent<Value>,
) -> anyhow::Result<()> {
    println!("{event:?}");

    // sequentially - unit test will pass
    // let result1 = app_client.operation1("something").await?;
    // let result2 = app_client.operation2("something").await?;

    let task1 = tokio::spawn(app_client.operation1("something"));
    let task2 = tokio::spawn(app_client.operation2("something"));

    let result1 = task1.await?;
    let result2 = task2.await?;
    println!("{result1:?}");
    println!("{result2:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::mock;

    #[tokio::test]
    async fn simeple_test() -> anyhow::Result<()> {
        // ARRANGE
        let json = r#"{"something": "something"}"#;
        let request: Value = serde_json::from_str(json).unwrap();
        let context = lambda_runtime::Context::default();
        let event = LambdaEvent::new(request, context);

        mock! {
            pub AppClient {}

            #[async_trait]
            impl AppInitialisation for AppClient {
                async fn operation1(&self, something: &str) -> anyhow::Result<String>;
                async fn operation2(&self, something: &str) -> anyhow::Result<Option<String>>;
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
        let result = function_handler(Box::leak(Box::new(mock)), event).await;

        // ASSERT
        assert_eq!(result.is_ok(), true);
        Ok(())
    }
}
