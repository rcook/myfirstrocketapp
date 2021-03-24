// "api" module defines public-facing structures that
// will typically be serialized to/from JSON and exposed
// to client via the REST API: these structures should
// not expose private information or implementation
// details such as database IDs

mod foo;

pub use self::foo::{Foo, NewFoo};
