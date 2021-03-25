use rocket::request::FromParam;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::{Serialize, Serializer};
use uuid::Uuid;

use crate::result::{Error, Result};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Guid(Uuid);

impl Guid {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }

    #[allow(dead_code)]
    pub fn parse_str(s: &str) -> Result<Self> {
        Ok(Uuid::parse_str(s).map(|x| Self(x))?)
    }
}

impl Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_hyphenated_ref().to_string())
    }
}

impl FromSql for Guid {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| match Uuid::parse_str(s) {
            Ok(x) => Ok(Self(x)),
            Err(_) => Err(FromSqlError::InvalidType),
        })
    }
}

impl ToSql for Guid {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.to_hyphenated_ref().to_string()))
    }
}

impl<'a> FromParam<'a> for Guid {
    type Error = Error;

    fn from_param(param: &'a str) -> std::result::Result<Self, Self::Error> {
        Ok(Guid(Uuid::parse_str(&param)?))
    }
}
