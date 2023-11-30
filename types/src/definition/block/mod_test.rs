#[cfg(test)]
mod test {
    use crate::definition::block::code::CodeBlock;
    use crate::definition::block::{Block, BlockType, CodeBlockType, Input, Output};
    use crate::definition::{DataType, Id};

    #[test]
    fn id_json_serialize() {
        let id: Id = Id::new("test_id");
        let expected = "\"test_id\"".to_string();
        let result = serde_json::to_string(&id);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn id_json_deserialize() {
        let string = "\"test_id\"".to_string();
        let expected = Id::new("test_id");
        let result = serde_json::from_str::<Id>(string.as_str());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

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
        let id = Id::new("js_id");
        let code_block_type = CodeBlockType::Js;
        let block_type = BlockType::Code;
        let inputs = vec![Input::new("input_id_1", DataType::Text)];
        let outputs = vec![Output::new("output_id_1", DataType::Text)];
        let code = "function f(x){return x+x}".to_string();
        let expected: Box<dyn Block> = Box::new(CodeBlock {
            id,
            block_type,
            code_block_type,
            inputs,
            outputs,
            code,
        });
        let payload: String = r#"{"type":"CodeBlock","id":"js_id","block_type":"Code","code_block_type":"Js","inputs":[{"name":"input_id_1","data_type":"Text"}],"outputs":[{"name":"output_id_1","data_type":"Text"}],"code":"function f(x){return x+x}"}"#.to_string();

        let result: Box<dyn Block> = serde_json::from_str(&payload).unwrap();
        assert_eq!(result.id(), expected.id());
        assert_eq!(result.block_type(), expected.block_type());
        assert_eq!(result.inputs(), expected.inputs());
        assert_eq!(result.outputs(), expected.outputs());
    }
}
