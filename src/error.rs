use aws_sdk_dynamodb::{self, types::SdkError};
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    InitError(String),
    ClientError(String),
    InternalError(String),
    SdkError(String),
}

impl std::error::Error for ApplicationError {}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::InitError(msg) => write!(f, "{msg}"),
            ApplicationError::ClientError(msg) => write!(f, "{msg}"),
            ApplicationError::InternalError(msg) => write!(f, "{msg}"),
            ApplicationError::SdkError(err) => write!(f, "{err}"),
        }
    }
}

impl<E> From<SdkError<E>> for ApplicationError
where
    E: error::Error,
{
    fn from(value: SdkError<E>) -> ApplicationError {
        ApplicationError::SdkError(format!("{value}"))
    }
}

impl From<Box<dyn std::error::Error + Sync + std::marker::Send>> for ApplicationError {
    fn from(value: Box<dyn std::error::Error + Sync + std::marker::Send>) -> Self {
        ApplicationError::InternalError(format!("{value:?}"))
    }
}
