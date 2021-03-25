use rocket::request::FromParam;
use rocket::http::RawStr;
use uuid::Uuid;

use crate::result::Error;

pub struct Guid(pub Uuid);

impl<'r> FromParam<'r> for Guid {
    type Error = Error;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        Ok(Guid(Uuid::parse_str(&param)?))
    }
}
