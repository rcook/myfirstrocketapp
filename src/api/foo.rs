use serde::Serialize;

use crate::object_model;

#[derive(Serialize)]
pub struct Foo {
    guid: String,
    name: String,
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

    use super::*;
    use crate::result::Result;

    #[test]
    fn test_blah() -> Result<()> {
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

    fn get_json_value(obj: Json<impl Serialize>) -> Result<serde_json::Value> {
        let s = serde_json::to_string(&obj.into_inner())?;
        Ok(serde_json::from_str(&s)?)
    }
}
