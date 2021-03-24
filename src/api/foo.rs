use serde::Serialize;

use crate::object_model;

#[derive(Serialize)]
pub struct Foo {
    guid: String,
    name: String,
}

impl std::convert::From<object_model::Foo> for Foo {
    fn from(foo: object_model::Foo) -> Self {
        Self {
            guid: foo.guid,
            name: foo.name,
        }
    }
}
