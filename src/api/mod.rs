// "api" module defines public-facing structures that
// will typically be serialized to/from JSON and exposed
// to client via the REST API: these structures should
// not expose private information or implementation
// details such as database IDs

pub mod foo;
pub mod foo_controller;
pub mod root_controller;
