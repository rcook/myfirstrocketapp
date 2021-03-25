use rocket::Route;
use rocket_contrib::json::Json;

use super::requests::{FooCreate, FooUpdate};
use super::responses::Foo;
use crate::app::foo;
use crate::db::Connection;
use crate::guid::Guid;
use crate::result::{not_found, Result};

pub fn make_routes() -> Vec<Route> {
    routes![create, delete, index, read, update]
}

#[post("/", format = "application/json", data = "<request>")]
async fn create(conn: Connection, request: Json<FooCreate>) -> Result<Json<Guid>> {
    conn.run(|conn| foo::create(&conn, &request.into_inner().name))
        .await
        .map(Json)
}

#[delete("/<guid>")]
async fn delete(conn: Connection, guid: Guid) -> Result<()> {
    conn.run(move |conn| foo::delete(&conn, guid)).await
}

#[get("/")]
async fn index(conn: Connection) -> Result<Json<Vec<Foo>>> {
    Ok(Json(
        conn.run(|conn| foo::index(&conn))
            .await?
            .into_iter()
            .map(Foo::from)
            .collect(),
    ))
}

#[get("/<guid>")]
async fn read(conn: Connection, guid: Guid) -> Result<Json<Foo>> {
    conn.run(move |conn| foo::read(&conn, guid))
        .await?
        .map_or_else(|| not_found(), Ok)
        .map(Foo::from)
        .map(Json)
}

#[put("/<guid>", format = "application/json", data = "<request>")]
async fn update(conn: Connection, guid: Guid, request: Json<FooUpdate>) -> Result<()> {
    conn.run(move |conn| foo::update(&conn, guid, &request.into_inner().name))
        .await
}
