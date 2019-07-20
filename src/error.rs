#[derive(Debug)]
pub enum Error {
    Http(::reqwest::Error),
    HtmlParseError,
    ParseIntError(::std::num::ParseIntError),
    ParseDateError(::chrono::format::ParseError),
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<::reqwest::Error> for Error {
    fn from(err: ::reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl From<::std::num::ParseIntError> for Error {
    fn from(err: ::std::num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

impl From<::chrono::format::ParseError> for Error {
    fn from(err: ::chrono::format::ParseError) -> Self {
        Error::ParseDateError(err)
    }
}
