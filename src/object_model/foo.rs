pub struct Foo {
    pub id: i32,
    pub guid: String,
    pub name: String,
}

impl Foo {
    pub fn new(id: i32, guid: &str, name: &str) -> Self {
        Self {
            id: id,
            guid: guid.into(),
            name: name.into(),
        }
    }
}
