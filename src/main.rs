#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod foo_controller;
mod result;
mod root_controller;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![root_controller::index, root_controller::can_fail],
        )
        .mount(
            "/foo",
            routes![foo_controller::index, foo_controller::can_fail],
        )
        .launch();
}
