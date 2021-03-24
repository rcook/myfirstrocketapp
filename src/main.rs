#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod result;

use crate::result::{internal_error_result, Result};

#[get("/")]
fn index() -> Result<&'static str> {
    Ok("Hello, world!")
}

#[get("/can-fail")]
fn can_fail() -> Result<&'static str> {
    internal_error_result("Facility", "Message")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, can_fail])
        .launch();
}
