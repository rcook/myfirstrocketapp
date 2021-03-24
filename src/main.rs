#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod api;
mod controllers;
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
                foo_controller::get,
                foo_controller::index
            ],
        )
        .launch();
}
