use rusqlite::Connection;

use crate::result::Result;

#[allow(dead_code)]
pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS foos (
            id          INTEGER PRIMARY KEY,
            guid        TEXT NOT NULL UNIQUE,
            name        TEXT NOT NULL
        );

        PRAGMA foreign_keys = ON;
        ",
    )?;
    Ok(())
}
