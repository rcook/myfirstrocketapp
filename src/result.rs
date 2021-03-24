use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Responder;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    NotFound,
    Internal(&'static str, String),
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'r> {
        match self {
            Error::NotFound => Err(Status::NotFound),
            Error::Internal(_facility, _message) => Err(Status::InternalServerError),
        }
    }
}

pub fn not_found_result<T>() -> Result<T> {
    Err(Error::NotFound)
}

pub fn internal_error<S>(facility: &'static str, message: S) -> Error
where
    S: Into<String>,
{
    Error::Internal(facility, message.into())
}

pub fn internal_error_result<T, S>(facility: &'static str, message: S) -> Result<T>
where
    S: Into<String>,
{
    Err(internal_error(facility, message))
}

impl std::convert::From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        internal_error("Rusqlite", error.to_string())
    }
}

impl std::convert::From<std::option::NoneError> for Error {
    fn from(_error: std::option::NoneError) -> Self {
        internal_error("Option", "Option was None")
    }
}
