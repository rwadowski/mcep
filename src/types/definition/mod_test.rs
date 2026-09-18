#[cfg(test)]
mod tests {
    use crate::types::definition::block::code::CodeBlock;
    use crate::types::definition::block::Block;
    use crate::types::definition::{DataType, Definition, NewDefinition, UpdateDefinition};
    use test_case::test_case;

    fn definition() -> Definition {
        Definition {
            id: 1,
            name: "my_definition".to_string(),
            version: "1.0.0".to_string(),
            body: serde_json::json!({
                "type": "CodeBlock",
                "inputs": [],
                "outputs": [],
                "source": "function f(x){return x}",
                "dependencies": []
            }),
            description: Some("desc".to_string()),
            help: Some("help".to_string()),
        }
    }

    #[test]
    fn definition_serialize() {
        // serde_json::Value serializes object keys in alphabetical order.
        let expected: String = r#"{
            "id": 1,
            "name": "my_definition",
            "version": "1.0.0",
            "body": {
                "dependencies": [],
                "inputs": [],
                "outputs": [],
                "source": "function f(x){return x}",
                "type": "CodeBlock"
            },
            "description": "desc",
            "help": "help"
        }"#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

        let result = serde_json::to_string(&definition()).unwrap();
        let result_normalized: String = result.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(result_normalized, expected);
    }

    #[test]
    fn definition_deserialize() {
        let payload = r#"{
            "id": 1,
            "name": "my_definition",
            "version": "1.0.0",
            "body": {
                "type": "CodeBlock",
                "inputs": [],
                "outputs": [],
                "source": "function f(x){return x}",
                "dependencies": []
            },
            "description": "desc",
            "help": "help"
        }"#;

        let result = serde_json::from_str::<Definition>(payload).unwrap();
        assert_eq!(result, definition());
    }

    #[test]
    fn definition_deserialize_optional_fields_missing() {
        let payload = r#"{
            "id": 1,
            "name": "my_definition",
            "version": "1.0.0",
            "body": {}
        }"#;

        let result = serde_json::from_str::<Definition>(payload).unwrap();
        assert_eq!(result.description, None);
        assert_eq!(result.help, None);
    }

    #[test]
    fn new_definition_deserialize() {
        let payload = r#"{
            "name": "my_definition",
            "version": "1.0.0",
            "body": {
                "type": "CodeBlock",
                "inputs": [],
                "outputs": [],
                "source": "function f(x){return x}",
                "dependencies": []
            },
            "description": "desc",
            "help": "help"
        }"#;

        let result = serde_json::from_str::<NewDefinition>(payload).unwrap();
        assert_eq!(result.name, "my_definition");
        assert_eq!(result.version, "1.0.0");
        assert_eq!(result.description, Some("desc".to_string()));
        assert_eq!(result.help, Some("help".to_string()));
        let body = result
            .body
            .as_any()
            .downcast_ref::<CodeBlock>()
            .expect("body should deserialize as a CodeBlock");
        assert_eq!(body.source, "function f(x){return x}");
    }

    #[test]
    fn new_definition_deserialize_missing_optional_fields() {
        let payload = r#"{
            "name": "my_definition",
            "version": "1.0.0",
            "body": {
                "type": "CodeBlock",
                "inputs": [],
                "outputs": [],
                "source": "function f(x){return x}",
                "dependencies": []
            }
        }"#;

        let result = serde_json::from_str::<NewDefinition>(payload).unwrap();
        assert_eq!(result.description, None);
        assert_eq!(result.help, None);
    }

    #[test]
    fn update_definition_deserialize_all_fields() {
        let payload = r#"{
            "id": 1,
            "version": "2.0.0",
            "name": "renamed",
            "body": "{}",
            "body_type": "CodeBlock",
            "description": "desc",
            "help": "help"
        }"#;

        let result = serde_json::from_str::<UpdateDefinition>(payload).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.version, Some("2.0.0".to_string()));
        assert_eq!(result.name, Some("renamed".to_string()));
        assert_eq!(result.body, Some("{}".to_string()));
        assert_eq!(result.body_type, Some("CodeBlock".to_string()));
        assert_eq!(result.description, Some("desc".to_string()));
        assert_eq!(result.help, Some("help".to_string()));
    }

    #[test]
    fn update_definition_deserialize_only_id_leaves_rest_none() {
        let payload = r#"{ "id": 1 }"#;

        let result = serde_json::from_str::<UpdateDefinition>(payload).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.version, None);
        assert_eq!(result.name, None);
        assert_eq!(result.body, None);
        assert_eq!(result.body_type, None);
        assert_eq!(result.description, None);
        assert_eq!(result.help, None);
    }

    #[test_case(DataType::Boolean, "\"Boolean\""; "serialization of boolean is correct")]
    #[test_case(DataType::UnsignedInt, "\"UnsignedInt\""; "serialization of unsigned int is correct"
    )]
    #[test_case(DataType::SignedInt, "\"SignedInt\""; "serialization of signed int is correct")]
    #[test_case(DataType::Float, "\"Float\""; "serialization of float is correct")]
    #[test_case(DataType::Text, "\"Text\""; "serialization of text is correct")]
    // #[test_case(DataType::Array(Box::from(DataType::Text)), "{\"Array\":\"Text\"}"; "serialization of array is correct")]
    // #[test_case(DataType::Map(Box::from(DataType::Text), Box::from(DataType::Text)), "{\"Map\":[\"Text\",\"Text\"]}"; "serialization of map is correct")]
    fn test_data_type_serialization(dt: DataType, expected: &str) {
        let result = serde_json::to_string(&dt);
        assert_eq!(true, result.is_ok());
        assert_eq!(expected.to_string(), result.unwrap());
    }

    #[test_case("\"Boolean\"", DataType::Boolean; "deserialization of boolean is correct")]
    #[test_case("\"UnsignedInt\"", DataType::UnsignedInt; "deserialization of unsigned int is correct"
    )]
    #[test_case("\"SignedInt\"", DataType::SignedInt; "deserialization of signed int is correct")]
    #[test_case("\"Float\"", DataType::Float; "deserialization of float is correct")]
    #[test_case("\"Text\"", DataType::Text; "deserialization of text is correct")]
    // #[test_case("{\"Array\":\"Text\"}", DataType::Array(Box::from(DataType::Text)); "deserialization of array is correct")]
    // #[test_case("{\"Map\":[\"Text\",\"Text\"]}", DataType::Map(Box::from(DataType::Text), Box::from(DataType::Text)); "deserialization of map is correct")]
    fn test_data_type_deserialization(data: &str, expected: DataType) {
        let result = serde_json::from_str::<DataType>(data);
        assert_eq!(true, result.is_ok());
        assert_eq!(expected, result.unwrap());
    }
}
