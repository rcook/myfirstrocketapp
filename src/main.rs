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

use crate::api::{foo_controller, root_controller};
use crate::connection_init::ConnectionInit;
use crate::db::Connection;

#[launch]
fn rocket() -> Rocket {
    rocket::ignite()
        .attach(Connection::fairing())
        .attach(ConnectionInit::fairing())
        .mount(
            "/",
            routes![root_controller::can_fail, root_controller::index],
        )
        .mount(
            "/foo",
            routes![
                foo_controller::create,
                foo_controller::delete,
                foo_controller::index,
                foo_controller::read,
                foo_controller::update
            ],
        )
}
