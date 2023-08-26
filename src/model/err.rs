use thiserror::Error;

/// 账户错误
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AccountError {
    #[error("InvalidFormat")]
    InvalidFormat,
    #[error("InvalidPhrase")]
    InvalidPhrase,
    #[error("InvalidPassword: {0}")]
    InvalidPassword(String),
    #[error("InvalidPassword: {0}")]
    InvalidAddress(String),
    #[error("InvalidSeed: {0}")]
    InvalidSeed(String),
    #[error("InvalidSeedLength")]
    InvalidSeedLength,
    #[error("InvalidPath: {0}")]
    InvalidPath(String),
}

/// 客户端错误
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ClientError {
    #[error("InvalidClient: {0}")]
    InvalidClient(String),
}
