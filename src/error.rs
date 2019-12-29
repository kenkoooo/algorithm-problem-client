#[derive(Debug)]
pub enum Error {
    Http(::surf::Exception),
    HtmlParseError,
    ParseIntError(::std::num::ParseIntError),
    ParseFloatError(::std::num::ParseFloatError),
    ParseDateError(::chrono::format::ParseError),
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<::surf::Exception> for Error {
    fn from(err: ::surf::Exception) -> Self {
        Error::Http(err)
    }
}

impl From<::std::num::ParseIntError> for Error {
    fn from(err: ::std::num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

impl From<::std::num::ParseFloatError> for Error {
    fn from(err: ::std::num::ParseFloatError) -> Self {
        Error::ParseFloatError(err)
    }
}

impl From<::chrono::format::ParseError> for Error {
    fn from(err: ::chrono::format::ParseError) -> Self {
        Error::ParseDateError(err)
    }
}
