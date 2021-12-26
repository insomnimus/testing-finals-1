use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Error(pub String);

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

impl<E: std::error::Error> From<E> for Error {
	fn from(e: E) -> Self {
		Self(e.to_string())
	}
}

pub type Result<T> = ::std::result::Result<T, Error>;
