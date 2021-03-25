use rusqlite::{params, Connection, OptionalExtension, Row, NO_PARAMS};
use uuid::Uuid;

use crate::object_model::Foo;
use crate::result::Result;

pub fn all(conn: &Connection) -> Result<Vec<Foo>> {
    let mut stmt = conn.prepare("SELECT id, guid, name FROM foos")?;
    let items = stmt
        .query_map(NO_PARAMS, from_row)?
        .collect::<rusqlite::Result<_>>()?;
    Ok(items)
}

pub fn by_guid(conn: &Connection, guid: &Uuid) -> Result<Option<Foo>> {
    let mut stmt = conn.prepare("SELECT id, guid, name FROM foos WHERE guid = ?1")?;
    let item_opt = stmt
        .query_row(params![guid.to_string()], from_row)
        .optional()?;
    Ok(item_opt)
}

pub fn delete(conn: &Connection, guid: &Uuid) -> Result<()> {
    conn.execute(
        "DELETE FROM foos WHERE guid = ?1",
        params![guid.to_string()],
    )?;
    Ok(())
}

pub fn insert(conn: &Connection, name: &str) -> Result<Uuid> {
    let guid = Uuid::new_v4();
    conn.execute(
        "INSERT INTO foos (guid, name) VALUES (?1, ?2)",
        params![guid.to_string(), name],
    )?;
    Ok(guid)
}

pub fn update(conn: &Connection, guid: &Uuid, name: &str) -> Result<()> {
    conn.execute(
        "UPDATE foos SET name = ?1 WHERE guid = ?2",
        params![name, guid.to_string()],
    )?;
    Ok(())
}

fn from_row(row: &Row) -> rusqlite::Result<Foo> {
    Ok(Foo::new(
        row.get(0)?,
        &row.get::<_, String>(1)?,
        &row.get::<_, String>(2)?,
    ))
}
