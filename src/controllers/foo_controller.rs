use rocket_contrib::json::Json;
use uuid::Uuid;

use crate::api::{Foo, NewFoo};
use crate::db::{foo, open_db};
use crate::result::{internal_error, not_found, Result};

#[get("/")]
pub fn index() -> Result<Json<Vec<Foo>>> {
    // Retrieve some internal Foo structures from the database
    let conn = open_db()?;
    let items = foo::all(&conn)?;

    // Translate the internal Foo structures to their API equivalents
    Ok(Json(items.into_iter().map(Foo::from).collect()))
}

#[get("/<guid_str>")]
pub fn read(guid_str: String) -> Result<Json<Foo>> {
    // Retrieve some internal Foo structures from the database
    let conn = open_db()?;
    let guid = Uuid::parse_str(&guid_str)?;
    let item = foo::by_guid(&conn, &guid)?;

    // Translate the internal Foo structure to its API equivalent
    item.map_or_else(|| not_found(), |x| Ok(Json(x.into())))
}

#[post("/", format = "application/json", data = "<foo>")]
pub fn create(foo: Json<NewFoo>) -> Result<Json<String>> {
    let conn = open_db()?;
    let item = foo.into_inner();
    let guid = foo::insert(&conn, &item.name)?;
    Ok(Json(guid.to_string()))
}

#[get("/can-fail")]
pub fn can_fail() -> Result<&'static str> {
    internal_error("facility", "message")
}
