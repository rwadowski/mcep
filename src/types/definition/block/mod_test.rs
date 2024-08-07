#[cfg(test)]
mod test {
    use crate::types::definition::block::code::CodeBlock;
    use crate::types::definition::block::{Block, BlockType, Input, Output};
    use crate::types::definition::DataType;

    #[test]
    fn input_json_serialize() {
        let name = "input_name";
        let dt = DataType::Text;
        let input = Input::new(name, dt);
        let expected: String = r#"
            {
                "name": "input_name",
                "data_type": "Text"
            }"#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
        let result = serde_json::to_string(&input);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn input_json_deserialize() {
        let name = "input_name";
        let dt = DataType::Text;
        let payload = r#"
            {
                "name": "input_name",
                "data_type": "Text"
            }
        "#;
        let expected = Input::new(name, dt);
        let result = serde_json::from_str::<Input>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn output_json_serialize() {
        let name = "output_name";
        let dt = DataType::Text;
        let output = Output::new(name, dt);
        let expected: String = r#"
            {
                "name": "output_name",
                "data_type": "Text"
            }"#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
        let result = serde_json::to_string(&output);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn output_json_deserialize() {
        let name = "output_name";
        let dt = DataType::Text;
        let payload = r#"
            {
                "name": "output_name",
                "data_type": "Text"
            }"#;
        let expected = Output::new(name, dt);
        let result = serde_json::from_str::<Output>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn dynamic_block_json_deserialize() {
        let inputs = vec![Input::new("input_id_1", DataType::Text)];
        let outputs = vec![Output::new("output_id_1", DataType::Text)];
        let code = "function f(x){return x+x}".to_string();
        let expected: Box<dyn Block> = Box::new(CodeBlock {
            inputs,
            outputs,
            source: code,
            dependencies: vec![],
        });
        let payload: String = r#"{"type":"CodeBlock","inputs":[{"name":"input_id_1","data_type":"Text"}],"outputs":[{"name":"output_id_1","data_type":"Text"}],"source":"function f(x){return x+x}","dependencies":[]}"#.to_string();

        let result: Box<dyn Block> = serde_json::from_str(&payload).unwrap();
        assert_eq!(result.block_type(), BlockType::CodeBlock);
        assert_eq!(result.inputs(), expected.inputs());
        assert_eq!(result.outputs(), expected.outputs());
    }
}
