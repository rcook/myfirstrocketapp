use rocket_contrib::json::Json;
use uuid::Uuid;

use crate::api::{Foo, FooCreate, FooUpdate};
use crate::db::{foo, open_db};
use crate::result::{not_found, Result};

#[post("/", format = "application/json", data = "<body>")]
pub fn create(body: Json<FooCreate>) -> Result<Json<String>> {
    let item = body.into_inner();
    let conn = open_db()?;
    let guid = foo::insert(&conn, &item.name)?;
    Ok(Json(guid.to_string()))
}

#[delete("/<guid_str>")]
pub fn delete(guid_str: String) -> Result<()> {
    let guid = Uuid::parse_str(&guid_str)?;
    let conn = open_db()?;
    foo::delete(&conn, &guid)?;
    Ok(())
}

#[get("/")]
pub fn index() -> Result<Json<Vec<Foo>>> {
    let conn = open_db()?;
    let items = foo::all(&conn)?;
    Ok(Json(items.into_iter().map(Foo::from).collect()))
}

#[get("/<guid_str>")]
pub fn read(guid_str: String) -> Result<Json<Foo>> {
    let guid = Uuid::parse_str(&guid_str)?;
    let conn = open_db()?;
    let item = foo::by_guid(&conn, &guid)?;
    item.map_or_else(|| not_found(), |x| Ok(Json(x.into())))
}

#[put("/<guid_str>", format = "application/json", data = "<body>")]
pub fn update(guid_str: String, body: Json<FooUpdate>) -> Result<()> {
    let guid = Uuid::parse_str(&guid_str)?;
    let item = body.into_inner();
    let conn = open_db()?;
    foo::update(&conn, &guid, &item.name)?;
    Ok(())
}
