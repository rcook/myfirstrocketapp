use dirs::home_dir;
use rusqlite::Connection;

use super::migrations::run_migrations;
use crate::result::Result;

pub fn open_db() -> Result<Connection> {
    let d = home_dir()?;
    let db_path = d.join("myfirstrocketapp.db");
    let conn = Connection::open(db_path)?;
    rusqlite::vtab::array::load_module(&conn)?;
    run_migrations(&conn)?;
    Ok(conn)
}
