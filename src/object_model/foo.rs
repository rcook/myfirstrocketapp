pub struct Foo {
    pub id: i64,
    pub guid: String,
    pub name: String,
}

impl Foo {
    pub fn new(id: i64, guid: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id,
            guid: guid.into(),
            name: name.into(),
        }
    }
}
