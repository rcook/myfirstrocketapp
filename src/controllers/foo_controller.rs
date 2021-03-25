use rocket_contrib::json::Json;

use crate::api::{Foo, FooCreate, FooUpdate};
use crate::db::{foo, Connection};
use crate::guid::Guid;
use crate::result::{not_found, Result};

#[post("/", format = "application/json", data = "<body>")]
pub async fn create(conn: Connection, body: Json<FooCreate>) -> Result<Json<Guid>> {
    conn.run(|conn| {
        let item = body.into_inner();
        let guid = foo::insert(&conn, &item.name)?;
        Ok(Json(guid))
    })
    .await
}

#[delete("/<guid>")]
pub async fn delete(conn: Connection, guid: Guid) -> Result<()> {
    conn.run(move |conn| {
        foo::delete(&conn, &guid)?;
        Ok(())
    })
    .await
}

#[get("/")]
pub async fn index(conn: Connection) -> Result<Json<Vec<Foo>>> {
    conn.run(|conn| {
        let items = foo::all(conn)?;
        Ok(Json(items.into_iter().map(Foo::from).collect()))
    })
    .await
}

#[get("/<guid>")]
pub async fn read(conn: Connection, guid: Guid) -> Result<Json<Foo>> {
    conn.run(move |conn| {
        let item = foo::by_guid(&conn, &guid)?;
        item.map_or_else(|| not_found(), |x| Ok(Json(x.into())))
    })
    .await
}

#[put("/<guid>", format = "application/json", data = "<body>")]
pub async fn update(conn: Connection, guid: Guid, body: Json<FooUpdate>) -> Result<()> {
    conn.run(move |conn| {
        let item = body.into_inner();
        foo::update(&conn, &guid, &item.name)?;
        Ok(())
    })
    .await
}
