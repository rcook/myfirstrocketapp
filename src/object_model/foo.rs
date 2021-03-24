pub struct Foo {
    pub id: i64,
    pub guid: String,
    pub name: String,
}

impl Foo {
    pub fn new(id: i64, guid: &str, name: &str) -> Self {
        Self {
            id: id,
            guid: guid.into(),
            name: name.into(),
        }
    }
}
