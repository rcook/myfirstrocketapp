#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod api;
mod app;
mod connection_init;
mod db;
mod guid;
mod object_model;
mod result;

use rocket::{ignite, Rocket};

use crate::connection_init::ConnectionInit;
use crate::db::Connection;

#[launch]
fn rocket() -> Rocket {
    ignite()
        .attach(Connection::fairing())
        .attach(ConnectionInit::fairing())
        .mount("/", api::root::make_routes())
        .mount("/foo", api::foo::make_routes())
}
