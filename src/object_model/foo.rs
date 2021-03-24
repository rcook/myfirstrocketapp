pub struct Foo {
    pub id: i32,
    pub name: String,
}

impl Foo {
    pub fn new(id: i32, name: &str) -> Self {
        Self {
            id: id,
            name: name.into(),
        }
    }
}
