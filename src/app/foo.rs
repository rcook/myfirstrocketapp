use rocket_contrib::json::Json;

use crate::api::{Foo, FooCreate, FooUpdate};
use crate::db;
use crate::guid::Guid;
use crate::result::{not_found, Result};

pub fn create(conn: &rusqlite::Connection, body: Json<FooCreate>) -> Result<Json<Guid>> {
    let item = body.into_inner();
    let guid = db::foo::insert(&conn, &item.name)?;
    Ok(Json(guid))
}

pub fn delete(conn: &rusqlite::Connection, guid: Guid) -> Result<()> {
    db::foo::delete(&conn, &guid)
}

pub fn index(conn: &rusqlite::Connection) -> Result<Json<Vec<Foo>>> {
    let items = db::foo::all(conn)?;
    Ok(Json(items.into_iter().map(Foo::from).collect()))
}

pub fn read(conn: &rusqlite::Connection, guid: Guid) -> Result<Json<Foo>> {
    let item = db::foo::by_guid(&conn, &guid)?;
    item.map_or_else(|| not_found(), |x| Ok(Json(x.into())))
}

pub fn update(conn: &rusqlite::Connection, guid: Guid, body: Json<FooUpdate>) -> Result<()> {
    let item = body.into_inner();
    db::foo::update(&conn, &guid, &item.name)
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

        assert!(index(&conn)?.is_empty());

        let guid0 = create(
            &conn,
            Json(FooCreate {
                name: String::from("NAME0"),
            }),
        )?
        .into_inner();
        let guid1 = create(
            &conn,
            Json(FooCreate {
                name: String::from("NAME1"),
            }),
        )?
        .into_inner();

        assert_eq!(2, index(&conn)?.len());
        assert_eq!("NAME0", read(&conn, guid0)?.into_inner().name);
        assert_eq!("NAME1", read(&conn, guid1)?.into_inner().name);

        delete(&conn, guid0)?;

        assert_eq!(1, index(&conn)?.len());
        assert!(read(&conn, guid0).is_err());
        assert_eq!("NAME1", read(&conn, guid1)?.into_inner().name);

        assert!(update(
            &conn,
            guid0,
            Json(FooUpdate {
                name: String::from("UPDATED-NAME0")
            })
        )
        .is_err());

        assert_eq!(1, index(&conn)?.len());
        assert!(read(&conn, guid0).is_err());
        assert_eq!("NAME1", read(&conn, guid1)?.into_inner().name);

        update(
            &conn,
            guid1,
            Json(FooUpdate {
                name: String::from("UPDATED-NAME1"),
            }),
        )?;

        assert_eq!(1, index(&conn)?.len());
        assert!(read(&conn, guid0).is_err());
        assert_eq!("UPDATED-NAME1", read(&conn, guid1)?.into_inner().name);

        Ok(())
    }
}
