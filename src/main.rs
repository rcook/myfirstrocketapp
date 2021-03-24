#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod result;
mod root_controller;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![root_controller::index, root_controller::can_fail],
        )
        .launch();
}
