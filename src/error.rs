use std::{error, fmt};

use reqwest::StatusCode;

/// A common error type returned from
#[derive(Debug)]
pub enum Error {
    /// An error induced by a failed reqwest
    ReqwestError(reqwest::Error),
    /// An error caused by an invalid response from the Nano API
    NanoError(StatusCode, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReqwestError(err) => write!(f, "Reqwest Error: {}", err),
            Error::NanoError(code, message) => write!(f, "NanoWrimo API Error: {} (status code {})", message, code.as_u16())
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ReqwestError(err) => Some(err),
            Error::NanoError(..) => None
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}
