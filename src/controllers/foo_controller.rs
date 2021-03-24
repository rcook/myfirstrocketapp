use rocket_contrib::json::Json;

use crate::api;
use crate::object_model;
use crate::result::{internal_error_result, Result};

// Could implement "From" or "TryFrom" instead
fn to_api(foo: &object_model::Foo) -> api::Foo {
    api::Foo::new(&foo.name)
}

#[get("/")]
pub fn index() -> Result<Json<Vec<api::Foo>>> {
    // Retrieve some internal Foo structures from the database
    let foos = vec![
        object_model::Foo::new(100, "a"),
        object_model::Foo::new(101, "b"),
    ];

    // Translate the internal Foo structures to their API equivalents
    Ok(Json(foos.iter().map(|x| to_api(x)).collect()))
}

#[get("/<guid>")]
pub fn get(guid: String) -> Result<Json<api::Foo>> {
    // Retrieve internal Foo structure from the database based on GUID
    // which is a public-facing identifier
    let foo = object_model::Foo::new(1000, &format!("Foo with GUID {}", guid));

    Ok(Json(to_api(&foo)))
}

#[get("/can-fail")]
pub fn can_fail() -> Result<&'static str> {
    internal_error_result("Facility", "Message")
}
