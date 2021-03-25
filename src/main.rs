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

use rocket::Rocket;

use crate::connection_init::ConnectionInit;
use crate::db::Connection;

#[launch]
fn rocket() -> Rocket {
    rocket::ignite()
        .attach(Connection::fairing())
        .attach(ConnectionInit::fairing())
        .mount("/", api::root::actions::make_routes())
        .mount("/foo", api::foo::actions::make_routes())
}
