#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]

#[macro_use]
extern crate rocket;

mod api;
mod controllers;
mod db;
mod guid;
mod object_model;
mod result;

use rocket::Rocket;

use crate::controllers::{foo_controller, root_controller};

#[launch]
fn rocket() -> Rocket {
    rocket::ignite()
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
