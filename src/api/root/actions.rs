use rocket::Route;

use crate::result::{internal_error, Result};

pub fn make_routes() -> Vec<Route> {
    routes![can_fail, index]
}

#[get("/")]
fn index() -> Result<&'static str> {
    Ok("Hello, world!")
}

#[get("/can-fail")]
fn can_fail() -> Result<&'static str> {
    internal_error("facility", "message")
}
