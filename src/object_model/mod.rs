// "object_model" module defines internal structures that
// should not be exposed via REST APIs etc.

mod foo;

pub use self::foo::Foo;
