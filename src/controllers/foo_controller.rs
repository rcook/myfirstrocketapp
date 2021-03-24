use rocket_contrib::json::Json;

use crate::api;
use crate::db::{foo, open_db};
use crate::object_model;
use crate::result::{internal_error_result, not_found_result, Result};

// Could implement "From" or "TryFrom" instead
fn to_api(foo: &object_model::Foo) -> api::Foo {
    api::Foo::new(&foo.guid, &foo.name)
}

#[get("/")]
pub fn index() -> Result<Json<Vec<api::Foo>>> {
    // Retrieve some internal Foo structures from the database
    let conn = open_db()?;
    let foos = foo::all(&conn)?;

    // Translate the internal Foo structures to their API equivalents
    Ok(Json(foos.iter().map(|x| to_api(x)).collect()))
}

#[get("/<guid>")]
pub fn get(guid: String) -> Result<Json<api::Foo>> {
    // Retrieve some internal Foo structures from the database
    let conn = open_db()?;
    let foo = foo::by_guid(&conn, &guid)?;

    // Translate the internal Foo structure to its API equivalent
    // TBD: Do this with or_else instead etc.
    match foo {
        Some(x) => Ok(Json(to_api(&x))),
        None => not_found_result(),
    }
}

#[get("/can-fail")]
pub fn can_fail() -> Result<&'static str> {
    internal_error_result("Facility", "Message")
}
