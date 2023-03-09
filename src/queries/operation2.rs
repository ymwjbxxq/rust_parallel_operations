use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait Operation2Query {
    async fn execute(&self, something: &str) -> anyhow::Result<Option<String>>;
}

#[derive(Debug, Clone, Builder)]
pub struct Operation2 {
    #[builder(setter(into))]
    table_name: String,

    #[builder(setter(into))]
    pub client: Client,
}

#[async_trait]
impl Operation2Query for Operation2 {
    async fn execute(&self, something: &str) -> anyhow::Result<Option<String>> {
        Ok(Some("something".to_string()))
    }
}
