use std;
use std::io;
use std::fmt;

use serde_json;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    Value(String),
}

pub type Result<T> = std::result::Result<T, self::Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
            Error::Value(ref string) => string.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::Value(ref string) => string,
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            Error::Value(..) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
