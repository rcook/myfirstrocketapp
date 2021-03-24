use serde::Serialize;

#[derive(Serialize)]
pub struct Foo {
    id: i32,
    name: String,
}

impl Foo {
    pub fn new(id: i32, name: &str) -> Self {
        Self {
            id: id,
            name: name.into(),
        }
    }
}
