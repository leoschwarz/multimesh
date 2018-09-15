use std::num::ParseFloatError;
use std::num::ParseIntError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO Error: {}", _0)]
    Io(::std::io::Error),

    #[fail(display = "Implementor of multimesh traits broke invariant, or internal bug: {}", _0)]
    BrokenInvariant(String),

    #[fail(display = "Syntax error: {}", _0)]
    Syntax(String),

    #[fail(display = "Other error (internal): {}", _0)]
    OtherInternal(Box<::std::error::Error + Send + Sync>),

    #[fail(display = "Other error (external): {}", _0)]
    OtherExternal(Box<::std::error::Error + Send + Sync>),
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::Syntax(format!("Parsing int failed: {}", e))
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Error::Syntax(format!("Parsing float failed: {}", e))
    }
}
