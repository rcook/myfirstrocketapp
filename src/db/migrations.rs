use rusqlite::Connection;

use crate::result::Result;

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        BEGIN TRANSACTION;
        CREATE TABLE IF NOT EXISTS foos (
            id          INTEGER PRIMARY KEY,
            guid        TEXT NOT NULL UNIQUE,
            name        TEXT NOT NULL
        );
        INSERT INTO foos (guid, name) VALUES ('guid-0', 'Foo 0')
            ON CONFLICT(guid) DO NOTHING;
        INSERT INTO foos (guid, name) VALUES ('guid-1', 'Foo 1')
            ON CONFLICT(guid) DO NOTHING;
        INSERT INTO foos (guid, name) VALUES ('guid-2', 'Foo 2')
            ON CONFLICT(guid) DO NOTHING;
        INSERT INTO foos (guid, name) VALUES ('guid-3', 'Foo 3')
            ON CONFLICT(guid) DO NOTHING;
        INSERT INTO foos (guid, name) VALUES ('guid-4', 'Foo 4')
            ON CONFLICT(guid) DO NOTHING;
        COMMIT;

        PRAGMA foreign_keys = ON;
        ",
    )?;
    Ok(())
}
