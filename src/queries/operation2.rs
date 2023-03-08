use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use typed_builder::TypedBuilder as Builder;
use crate::error::ApplicationError;

#[async_trait]
pub trait Operation2Query {
    async fn execute(&self, something: &str) -> Result<Option<String>, ApplicationError>;
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
    async fn execute(&self, something: &str) -> Result<Option<String>, ApplicationError> {
        Ok(Some("something".to_string()))
    }
}
