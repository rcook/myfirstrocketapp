use rocket_contrib::json::Json;

use crate::api::{Foo, FooCreate, FooUpdate};
use crate::db::{foo, Connection};
use crate::guid::Guid;
use crate::result::{not_found, Result};

#[post("/", format = "application/json", data = "<body>")]
pub async fn create(conn: Connection, body: Json<FooCreate>) -> Result<Json<Guid>> {
    conn.run(|conn| create_inner(&conn, body)).await
}
fn create_inner(conn: &rusqlite::Connection, body: Json<FooCreate>) -> Result<Json<Guid>> {
    let item = body.into_inner();
    let guid = foo::insert(&conn, &item.name)?;
    Ok(Json(guid))
}

#[delete("/<guid>")]
pub async fn delete(conn: Connection, guid: Guid) -> Result<()> {
    conn.run(move |conn| delete_inner(&conn, guid)).await
}
fn delete_inner(conn: &rusqlite::Connection, guid: Guid) -> Result<()> {
    foo::delete(&conn, &guid)
}

#[get("/")]
pub async fn index(conn: Connection) -> Result<Json<Vec<Foo>>> {
    conn.run(|conn| index_inner(&conn)).await
}
fn index_inner(conn: &rusqlite::Connection) -> Result<Json<Vec<Foo>>> {
    let items = foo::all(conn)?;
    Ok(Json(items.into_iter().map(Foo::from).collect()))
}

#[get("/<guid>")]
pub async fn read(conn: Connection, guid: Guid) -> Result<Json<Foo>> {
    conn.run(move |conn| read_inner(&conn, guid)).await
}
fn read_inner(conn: &rusqlite::Connection, guid: Guid) -> Result<Json<Foo>> {
    let item = foo::by_guid(&conn, &guid)?;
    item.map_or_else(|| not_found(), |x| Ok(Json(x.into())))
}

#[put("/<guid>", format = "application/json", data = "<body>")]
pub async fn update(conn: Connection, guid: Guid, body: Json<FooUpdate>) -> Result<()> {
    conn.run(move |conn| update_inner(&conn, guid, body)).await
}
fn update_inner(conn: &rusqlite::Connection, guid: Guid, body: Json<FooUpdate>) -> Result<()> {
    let item = body.into_inner();
    foo::update(&conn, &guid, &item.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::FooCreate;
    use crate::db::run_migrations;
    use crate::result::Result;

    #[test]
    fn test_basics_() -> Result<()> {
        let conn = rusqlite::Connection::open_in_memory()?;
        run_migrations(&conn)?;

        assert!(index_inner(&conn)?.is_empty());

        let guid0 = create_inner(
            &conn,
            Json(FooCreate {
                name: String::from("NAME0"),
            }),
        )?
        .into_inner();
        let guid1 = create_inner(
            &conn,
            Json(FooCreate {
                name: String::from("NAME1"),
            }),
        )?
        .into_inner();

        assert_eq!(2, index_inner(&conn)?.len());
        assert_eq!("NAME0", read_inner(&conn, guid0)?.into_inner().name);
        assert_eq!("NAME1", read_inner(&conn, guid1)?.into_inner().name);

        delete_inner(&conn, guid0)?;

        assert_eq!(1, index_inner(&conn)?.len());
        assert!(read_inner(&conn, guid0).is_err());
        assert_eq!("NAME1", read_inner(&conn, guid1)?.into_inner().name);

        assert!(update_inner(
            &conn,
            guid0,
            Json(FooUpdate {
                name: String::from("UPDATED-NAME0")
            })
        )
        .is_err());

        assert_eq!(1, index_inner(&conn)?.len());
        assert!(read_inner(&conn, guid0).is_err());
        assert_eq!("NAME1", read_inner(&conn, guid1)?.into_inner().name);

        update_inner(
            &conn,
            guid1,
            Json(FooUpdate {
                name: String::from("UPDATED-NAME1"),
            }),
        )?;

        assert_eq!(1, index_inner(&conn)?.len());
        assert!(read_inner(&conn, guid0).is_err());
        assert_eq!("UPDATED-NAME1", read_inner(&conn, guid1)?.into_inner().name);

        Ok(())
    }
}
