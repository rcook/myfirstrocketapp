use rocket_contrib::json::Json;

use crate::api::{Foo, FooCreate, FooUpdate};
use crate::app::foo;
use crate::db::Connection;
use crate::guid::Guid;
use crate::result::Result;

#[post("/", format = "application/json", data = "<body>")]
pub async fn create(conn: Connection, body: Json<FooCreate>) -> Result<Json<Guid>> {
    conn.run(|conn| foo::create(&conn, body)).await
}

#[delete("/<guid>")]
pub async fn delete(conn: Connection, guid: Guid) -> Result<()> {
    conn.run(move |conn| foo::delete(&conn, guid)).await
}

#[get("/")]
pub async fn index(conn: Connection) -> Result<Json<Vec<Foo>>> {
    conn.run(|conn| foo::index(&conn)).await
}

#[get("/<guid>")]
pub async fn read(conn: Connection, guid: Guid) -> Result<Json<Foo>> {
    conn.run(move |conn| foo::read(&conn, guid)).await
}

#[put("/<guid>", format = "application/json", data = "<body>")]
pub async fn update(conn: Connection, guid: Guid, body: Json<FooUpdate>) -> Result<()> {
    conn.run(move |conn| foo::update(&conn, guid, body)).await
}
