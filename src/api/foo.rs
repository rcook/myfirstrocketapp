use serde::Serialize;

#[derive(Serialize)]
pub struct Foo {
    name: String,
}

impl Foo {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}
