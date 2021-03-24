use rocket_contrib::json::Json;

use crate::foo::Foo;
use crate::result::{internal_error_result, Result};

#[get("/")]
pub fn index() -> Result<Json<Vec<Foo>>> {
    Ok(Json(vec![Foo::new(100, "a"), Foo::new(101, "b")]))
}

#[get("/<id>")]
pub fn get(id: i32) -> Result<Json<Foo>> {
    Ok(Json(Foo::new(id, "a")))
}

#[get("/can-fail")]
pub fn can_fail() -> Result<&'static str> {
    internal_error_result("Facility", "Message")
}
