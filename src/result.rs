use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Responder;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Client(String),
    Internal(&'static str, String),
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'r> {
        match self {
            Error::Client(_message) => Err(Status::BadRequest),
            Error::Internal(_facility, _message) => Err(Status::InternalServerError),
        }
    }
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
