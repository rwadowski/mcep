#[cfg(test)]
mod test {
    use crate::types::definition::block::code::CodeBlock;
    use crate::types::definition::block::{Block, BlockType, Input, Output};
    use crate::types::definition::DataType;
    #[test]
    fn js_json_serialize() {
        let inputs = vec![Input::new("input_id_1", DataType::Text)];
        let outputs = vec![Output::new("output_id_1", DataType::Text)];
        let code = "function f(x){return x+x}".to_string();
        let js = CodeBlock {
            inputs,
            outputs,
            source: code,
            dependencies: vec![],
        };
        let js_string: String = r#"
            {
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
        "#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
        let expected = js_string.replace("functionf(x){returnx+x}", "function f(x){return x+x}");
        let boxed: Box<dyn Block> = Box::new(js);
        let result = serde_json::to_string(boxed.as_ref());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn js_json_deserialize() {
        let inputs = vec![Input::new("input_id_1", DataType::Text)];
        let outputs = vec![Output::new("output_id_1", DataType::Text)];
        let code = "function f(x){return x+x}".to_string();
        let expected = CodeBlock {
            inputs,
            outputs,
            source: code,
            dependencies: vec![],
        };
        let payload: String = r#"
            {
                "type": "CodeBlock",
                "code_block_type": "Js",
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
        "#
        .to_string();
        let result = serde_json::from_str::<CodeBlock>(&payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
