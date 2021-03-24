use rocket_contrib::json::Json;
use rusqlite::{params, OptionalExtension, Row, NO_PARAMS};

use crate::api;
use crate::db::open_db;
use crate::object_model;
use crate::result::{internal_error_result, not_found_result, Result};

// Could implement "From" or "TryFrom" instead
fn to_api(foo: &object_model::Foo) -> api::Foo {
    api::Foo::new(&foo.guid, &foo.name)
}

fn to_foo(row: &Row) -> rusqlite::Result<object_model::Foo> {
    Ok(object_model::Foo::new(
        row.get(0)?,
        &row.get::<_, String>(1)?,
        &row.get::<_, String>(2)?,
    ))
}

#[get("/")]
pub fn index() -> Result<Json<Vec<api::Foo>>> {
    // Retrieve some internal Foo structures from the database
    let conn = open_db()?;
    let mut stmt = conn.prepare("SELECT id, guid, name FROM foos")?;
    let foos: Vec<_> = stmt
        .query_map(NO_PARAMS, to_foo)?
        .collect::<rusqlite::Result<_>>()?;

    // Translate the internal Foo structures to their API equivalents
    Ok(Json(foos.iter().map(|x| to_api(x)).collect()))
}

#[get("/<guid>")]
pub fn get(guid: String) -> Result<Json<api::Foo>> {
    // Retrieve some internal Foo structures from the database
    let conn = open_db()?;
    let mut stmt = conn.prepare("SELECT id, guid, name FROM foos WHERE guid = ?1")?;
    let foo = stmt.query_row(params![guid], to_foo).optional()?;

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
