use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};
use typed_builder::TypedBuilder as Builder;

use crate::{
    error::ApplicationError,
    queries::{
        operation1::{Operation1, Operation1Query},
        operation2::{Operation2, Operation2Query},
    },
};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AppInitialisation: Send + Sync {
    async fn operation1(&self, something: &str) -> Result<String, ApplicationError>;

    async fn operation2(&self, something: &str) -> Result<Option<String>, ApplicationError>;
}

#[derive(Debug, Clone, Builder)]
pub struct AppClient {
    #[builder(setter(into))]
    pub operation1: Operation1,

    #[builder(setter(into))]
    pub operation2: Operation2,
}

#[async_trait]
impl AppInitialisation for AppClient {
    async fn operation1(&self, something: &str) -> Result<String, ApplicationError> {
        self.operation1.execute(something).await
    }

    async fn operation2(&self, something: &str) -> Result<Option<String>, ApplicationError> {
        self.operation2.execute(something).await
    }
}
