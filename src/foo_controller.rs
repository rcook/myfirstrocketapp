use crate::result::{internal_error_result, Result};

#[get("/")]
pub fn index() -> Result<&'static str> {
    Ok("Hello, world!")
}

#[get("/can-fail")]
pub fn can_fail() -> Result<&'static str> {
    internal_error_result("Facility", "Message")
}
