use rocket_contrib::json::Json;

use crate::api::Foo;
use crate::db::{foo, open_db};
use crate::result::{internal_error_result, not_found_result, Result};

#[get("/")]
pub fn index() -> Result<Json<Vec<Foo>>> {
    // Retrieve some internal Foo structures from the database
    let conn = open_db()?;
    let items = foo::all(&conn)?;

    // Translate the internal Foo structures to their API equivalents
    Ok(Json(items.into_iter().map(Foo::from).collect()))
}

#[get("/<guid>")]
pub fn get(guid: String) -> Result<Json<Foo>> {
    // Retrieve some internal Foo structures from the database
    let conn = open_db()?;
    let item = foo::by_guid(&conn, &guid)?;

    // Translate the internal Foo structure to its API equivalent
    item.map_or_else(|| not_found_result(), |x| Ok(Json(x.into())))
}

#[get("/can-fail")]
pub fn can_fail() -> Result<&'static str> {
    internal_error_result("Facility", "Message")
}
