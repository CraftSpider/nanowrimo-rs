use crate::NanoError;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    NanoError(StatusCode, NanoError),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}
