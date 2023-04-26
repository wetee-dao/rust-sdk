use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AccountError {
    #[error("InvalidFormat")]
    InvalidFormat,
    #[error("InvalidPhrase")]
    InvalidPhrase,
    #[error("InvalidPassword: {0}")]
    InvalidPassword(String),
    #[error("InvalidSeed: {0}")]
    InvalidSeed(String),
    #[error("InvalidSeedLength")]
    InvalidSeedLength,
    #[error("InvalidPath: {0}")]
    InvalidPath(String),
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ClientError {
    #[error("InvalidClient: {0}")]
    InvalidClient(String),
}
