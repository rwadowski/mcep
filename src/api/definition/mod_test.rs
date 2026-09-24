#[cfg(test)]
mod tests {
    use crate::types::definition::block::{Block, BlockType, Input, Output};
    use crate::types::definition::{DataType, NewDefinition};

    #[test]
    fn test_new_definition_deserialize_body() {
        let payload = r#"{
            "name": "definition test",
            "version": "1.0.0",
            "description": "a test definition",
            "help": "some help text",
            "body": {
                "type": "CodeBlock",
                "inputs": [
                    {
                        "name": "input_id_1",
                        "data_type": "Text"
                    }
                ],
                "outputs": [
                    {
                        "name": "output_id_1",
                        "data_type": "Text"
                    }
                ],
                "source": "function f(x){return x+x}",
                "dependencies": []
            }
        }"#;

        let result = serde_json::from_str::<NewDefinition>(payload).unwrap();

        assert_eq!(result.name, "definition test");
        assert_eq!(result.version, "1.0.0");
        assert_eq!(result.description, Some("a test definition".to_string()));
        assert_eq!(result.help, Some("some help text".to_string()));
        assert_eq!(result.body.block_type(), BlockType::CodeBlock);
        assert_eq!(
            result.body.inputs(),
            vec![Input::new("input_id_1", DataType::Text)]
        );
        assert_eq!(
            result.body.outputs(),
            vec![Output::new("output_id_1", DataType::Text)]
        );
    }
}
