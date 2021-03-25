use rusqlite::Connection;

use crate::db;
use crate::guid::Guid;
use crate::object_model::Foo;
use crate::result::Result;

pub fn create(conn: &Connection, name: &str) -> Result<Guid> {
    Ok(db::foo::insert(&conn, name)?)
}

pub fn delete(conn: &Connection, guid: Guid) -> Result<()> {
    db::foo::delete(&conn, &guid)
}

pub fn index(conn: &Connection) -> Result<Vec<Foo>> {
    Ok(db::foo::all(conn)?)
}

pub fn read(conn: &Connection, guid: Guid) -> Result<Option<Foo>> {
    Ok(db::foo::by_guid(&conn, &guid)?)
}

pub fn update(conn: &Connection, guid: Guid, name: &str) -> Result<()> {
    db::foo::update(&conn, &guid, name)
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::*;
    use crate::db::run_migrations;
    use crate::result::Result;

    #[test]
    fn test_basics_() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        assert!(index(&conn)?.is_empty());

        let guid0 = create(&conn, "NAME0")?;
        let guid1 = create(&conn, "NAME1")?;

        assert_eq!(2, index(&conn)?.len());
        assert_eq!("NAME0", read(&conn, guid0)??.name);
        assert_eq!("NAME1", read(&conn, guid1)??.name);

        delete(&conn, guid0)?;

        assert_eq!(1, index(&conn)?.len());
        assert!(read(&conn, guid0)?.is_none());
        assert_eq!("NAME1", read(&conn, guid1)??.name);

        assert!(update(&conn, guid0, "UPDATED-NAME0").is_err());

        assert_eq!(1, index(&conn)?.len());
        assert!(read(&conn, guid0)?.is_none());
        assert_eq!("NAME1", read(&conn, guid1)??.name);

        update(&conn, guid1, "UPDATED-NAME1")?;

        assert_eq!(1, index(&conn)?.len());
        assert!(read(&conn, guid0)?.is_none());
        assert_eq!("UPDATED-NAME1", read(&conn, guid1)??.name);

        Ok(())
    }
}
