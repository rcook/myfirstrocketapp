#[database("sqlite3")]
pub struct Connection(pub rusqlite::Connection);
