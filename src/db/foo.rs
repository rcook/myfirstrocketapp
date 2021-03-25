use rusqlite::{params, Connection, OptionalExtension, Row, NO_PARAMS};

use crate::guid::Guid;
use crate::object_model::Foo;
use crate::result::{internal_error, Result};

pub fn all(conn: &Connection) -> Result<Vec<Foo>> {
    let mut stmt = conn.prepare("SELECT id, guid, name FROM foos")?;
    let items = stmt
        .query_map(NO_PARAMS, from_row)?
        .collect::<rusqlite::Result<_>>()?;
    Ok(items)
}

pub fn by_guid(conn: &Connection, guid: &Guid) -> Result<Option<Foo>> {
    let mut stmt = conn.prepare("SELECT id, guid, name FROM foos WHERE guid = ?1")?;
    let item_opt = stmt.query_row(params![guid], from_row).optional()?;
    Ok(item_opt)
}

pub fn delete(conn: &Connection, guid: &Guid) -> Result<()> {
    conn.execute("DELETE FROM foos WHERE guid = ?1", params![guid])?;
    Ok(())
}

pub fn insert(conn: &Connection, name: &str) -> Result<Guid> {
    let guid = Guid::new_v4();
    conn.execute(
        "INSERT INTO foos (guid, name) VALUES (?1, ?2)",
        params![guid, name],
    )?;
    Ok(guid)
}

pub fn update(conn: &Connection, guid: &Guid, name: &str) -> Result<()> {
    let count = conn.execute(
        "UPDATE foos SET name = ?1 WHERE guid = ?2",
        params![name, guid],
    )?;
    match count {
        0 => internal_error("rusqlite", "no row was updated"),
        1 => Ok(()),
        _ => internal_error("rusqlite", "more than one row was updated"),
    }
}

fn from_row(row: &Row) -> rusqlite::Result<Foo> {
    Ok(Foo::new(
        row.get(0)?,
        row.get(1)?,
        &row.get::<_, String>(2)?,
    ))
}
