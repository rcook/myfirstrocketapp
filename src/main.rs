#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]

#[macro_use]
extern crate rocket;

mod api;
mod controllers;
mod db;
mod object_model;
mod result;

use crate::controllers::{foo_controller, root_controller};

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![root_controller::can_fail, root_controller::index],
        )
        .mount(
            "/foo",
            routes![
                foo_controller::can_fail,
                foo_controller::create,
                foo_controller::index,
                foo_controller::read
            ],
        )
        .launch();
}
