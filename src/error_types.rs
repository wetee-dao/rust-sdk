use std::{
	fmt::{self, Display},
	error,
};

#[derive(Clone, Debug)]
pub enum Error {
	WrongAcount,
	SubxtError(&'static str),
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::WrongAcount => write!(f, "错误的账户"),
			Self::SubxtError(e) => write!(f, "Error from subxt crate: {}", e),
		}
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Self::WrongAcount => "错误的账户",
			Self::SubxtError(e) => e,
		}
	}
}
