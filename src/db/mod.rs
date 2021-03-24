pub mod foo;

mod connection;
mod migrations;

pub use self::connection::open_db;
