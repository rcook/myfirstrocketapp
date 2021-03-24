use serde::{Deserialize, Serialize};

use crate::object_model;

#[derive(Serialize)]
pub struct Foo {
    guid: String,
    name: String,
}

#[derive(Deserialize)]
pub struct NewFoo {
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
    fn test_from() {
        let result = Foo::from(object_model::Foo::new(100, "GUID", "NAME"));
        assert_eq!("GUID", result.guid);
        assert_eq!("NAME", result.name)
    }

    #[test]
    fn test_into() {
        let result: Foo = object_model::Foo::new(100, "GUID", "NAME").into();
        assert_eq!("GUID", result.guid);
        assert_eq!("NAME", result.name)
    }

    #[test]
    fn test_layout() -> Result<()> {
        let obj = Json(Foo {
            guid: "GUID".into(),
            name: "NAME".into(),
        });
        let value = get_json_value(obj)?;
        assert_eq!(
            json!({
                "guid": "GUID",
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
