mod actions;
mod requests;
mod responses;

pub use self::actions::make_routes;
pub use self::requests::{FooCreate, FooUpdate};
pub use self::responses::Foo;
