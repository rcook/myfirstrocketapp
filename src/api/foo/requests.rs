use serde::Deserialize;

#[derive(Deserialize)]
pub struct FooCreate {
    pub name: String,
}

#[derive(Deserialize)]
pub struct FooUpdate {
    pub name: String,
}
