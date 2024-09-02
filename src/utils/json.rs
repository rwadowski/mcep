use serde::Serialize;
use serde_json::{Error, Value};
#[derive(Serialize)]
struct TypeTag {
    r#type: String,
}

#[derive(Serialize)]
struct Tagged<T: Serialize> {
    #[serde(flatten)]
    tag: TypeTag,
    #[serde(flatten)]
    value: T,
}

pub fn serialize_to_value_with_type_tag<T: Serialize>(
    value: &T,
    tag: &str,
) -> Result<Value, Error> {
    let tagged = Tagged {
        tag: TypeTag {
            r#type: tag.to_string(),
        },
        value,
    };
    serde_json::to_value(&tagged)
}

pub fn serialize_to_string_with_type_tag<T: Serialize>(
    value: &T,
    tag: &str,
) -> Result<String, Error> {
    let tagged = Tagged {
        tag: TypeTag {
            r#type: tag.to_string(),
        },
        value,
    };
    serde_json::to_string(&tagged)
}
