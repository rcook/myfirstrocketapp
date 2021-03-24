use serde::Serialize;

#[derive(Serialize)]
pub struct Foo {
    guid: String,
    name: String,
}

impl Foo {
    pub fn new(guid: &str, name: &str) -> Self {
        Self {
            guid: guid.into(),
            name: name.into(),
        }
    }
}
