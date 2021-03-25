use serde::{Deserialize, Serialize};

use crate::guid::Guid;
use crate::object_model;

#[derive(Serialize)]
pub struct Foo {
    guid: Guid,
    name: String,
}

#[derive(Deserialize)]
pub struct FooCreate {
    pub name: String,
}

#[derive(Deserialize)]
pub struct FooUpdate {
    pub name: String,
}

impl std::convert::From<object_model::Foo> for Foo {
    fn from(foo: object_model::Foo) -> Self {
        Self {
            guid: foo.guid,
            name: foo.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use rocket_contrib::json::Json;
    use serde_json::json;

    use self::helpers::*;
    use super::*;
    use crate::result::Result;

    #[test]
    fn test_from() -> Result<()> {
        let guid = Guid::parse_str("e1fecf7f-4de7-4e7a-8c92-440b0b542030")?;
        let result = Foo::from(object_model::Foo::new(100, guid, "NAME"));
        assert_eq!(guid, result.guid);
        assert_eq!("NAME", result.name);
        Ok(())
    }

    #[test]
    fn test_into() -> Result<()> {
        let guid = Guid::parse_str("e1fecf7f-4de7-4e7a-8c92-440b0b542030")?;
        let result: Foo = object_model::Foo::new(100, guid, "NAME").into();
        assert_eq!(guid, result.guid);
        assert_eq!("NAME", result.name);
        Ok(())
    }

    #[test]
    fn test_layout() -> Result<()> {
        let guid = Guid::parse_str("e1fecf7f-4de7-4e7a-8c92-440b0b542030")?;
        let obj = Json(Foo {
            guid: guid,
            name: String::from("NAME"),
        });
        let value = get_json_value(obj)?;
        assert_eq!(
            json!({
                "guid": "e1fecf7f-4de7-4e7a-8c92-440b0b542030",
                "name": "NAME"
            }),
            value
        );
        Ok(())
    }

    mod helpers {
        use rocket_contrib::json::Json;
        use serde::Serialize;

        use crate::result::Result;

        pub fn get_json_value(obj: Json<impl Serialize>) -> Result<serde_json::Value> {
            let s = serde_json::to_string(&obj.into_inner())?;
            Ok(serde_json::from_str(&s)?)
        }
    }
}
