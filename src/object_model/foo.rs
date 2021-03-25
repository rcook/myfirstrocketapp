use crate::guid::Guid;

pub struct Foo {
    pub id: i64,
    pub guid: Guid,
    pub name: String,
}

impl Foo {
    pub fn new(id: i64, guid: Guid, name: impl Into<String>) -> Self {
        Self {
            id: id,
            guid: guid,
            name: name.into(),
        }
    }
}
