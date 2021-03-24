use rusqlite::{params, Connection, OptionalExtension, Row, NO_PARAMS};

use crate::object_model::Foo;
use crate::result::Result;

pub fn all(conn: &Connection) -> Result<Vec<Foo>> {
    let mut stmt = conn.prepare("SELECT id, guid, name FROM foos")?;
    let items = stmt
        .query_map(NO_PARAMS, from_row)?
        .collect::<rusqlite::Result<_>>()?;
    Ok(items)
}

pub fn by_guid(conn: &Connection, guid: &str) -> Result<Option<Foo>> {
    let mut stmt = conn.prepare("SELECT id, guid, name FROM foos WHERE guid = ?1")?;
    let item_opt = stmt.query_row(params![guid], from_row).optional()?;
    Ok(item_opt)
}

fn from_row(row: &Row) -> rusqlite::Result<Foo> {
    Ok(Foo::new(
        row.get(0)?,
        &row.get::<_, String>(1)?,
        &row.get::<_, String>(2)?,
    ))
}
