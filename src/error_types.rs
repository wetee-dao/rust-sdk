use std::{
    error,
    fmt::{self, Display},
};

#[derive(Clone, Debug)]
pub enum Error {
    WrongAcount,
    SdkError(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::WrongAcount => write!(f, "错误的账户"),
            Self::SdkError(e) => write!(f, "Error from subxt crate: {}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Self::WrongAcount => "错误的账户",
            Self::SdkError(e) => e,
        }
    }
}
