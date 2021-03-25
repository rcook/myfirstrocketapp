use rocket_contrib::json::Json;

use crate::api::{Foo, FooCreate, FooUpdate};
use crate::db::{foo, open_db};
use crate::guid::Guid;
use crate::result::{not_found, Result};

#[post("/", format = "application/json", data = "<body>")]
pub fn create(body: Json<FooCreate>) -> Result<Json<Guid>> {
    let item = body.into_inner();
    let conn = open_db()?;
    let guid = foo::insert(&conn, &item.name)?;
    Ok(Json(guid))
}

#[delete("/<guid>")]
pub fn delete(guid: Guid) -> Result<()> {
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

#[get("/<guid>")]
pub fn read(guid: Guid) -> Result<Json<Foo>> {
    let conn = open_db()?;
    let item = foo::by_guid(&conn, &guid)?;
    item.map_or_else(|| not_found(), |x| Ok(Json(x.into())))
}

#[put("/<guid>", format = "application/json", data = "<body>")]
pub fn update(guid: Guid, body: Json<FooUpdate>) -> Result<()> {
    let item = body.into_inner();
    let conn = open_db()?;
    foo::update(&conn, &guid, &item.name)?;
    Ok(())
}
