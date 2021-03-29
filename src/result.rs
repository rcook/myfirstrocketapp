use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Responder;
use std::convert::From;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    NotFound,
    Internal(&'static str, String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "NotFound"),
            Error::Internal(facility, message) => {
                write!(f, "facility={} message={}", facility, message)
            }
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            Error::NotFound => Err(Status::NotFound),
            Error::Internal(_facility, _message) => Err(Status::InternalServerError),
        }
    }
}

pub fn not_found<T>() -> Result<T> {
    Err(Error::NotFound)
}

pub fn internal_error<T>(facility: &'static str, message: impl Into<String>) -> Result<T> {
    Err(Error::Internal(facility, message.into()))
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Error::Internal("jsonwebtoken", error.to_string())
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::Internal("rusqlite", error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Internal("serde", error.to_string())
    }
}

impl From<std::option::NoneError> for Error {
    fn from(_error: std::option::NoneError) -> Self {
        Error::Internal("option", String::from("Option was None"))
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(error: std::time::SystemTimeError) -> Self {
        Error::Internal("time", error.to_string())
    }
}

impl From<uuid::Error> for Error {
    fn from(error: uuid::Error) -> Self {
        Error::Internal("uuid", error.to_string())
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(error: std::num::TryFromIntError) -> Self {
        Error::Internal("num", error.to_string())
    }
}
