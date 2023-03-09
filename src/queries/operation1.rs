use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait Operation1Query {
    async fn execute(&self, something: &str) -> anyhow::Result<String>;
}

#[derive(Debug, Clone, Builder)]
pub struct Operation1 {
    #[builder(setter(into))]
    table_name: String,

    #[builder(setter(into))]
    pub client: Client,
}

#[async_trait]
impl Operation1Query for Operation1 {
    async fn execute(&self, something: &str) -> anyhow::Result<String> {
        Ok("something".to_string())
    }
}
