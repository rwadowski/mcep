#[cfg(test)]
mod test {
    use crate::definition::block::code::CodeBlock;
    use crate::definition::block::{BlockType, CodeBlockType, Input, Output};
    use crate::definition::{DataType, Id};
    #[test]
    fn js_json_serialize() {
        let id = Id::new("js_id");
        let block_type = BlockType::Code;
        let code_block_type = CodeBlockType::Js;
        let inputs = vec![Input::new("input_id_1", DataType::Text)];
        let outputs = vec![Output::new("output_id_1", DataType::Text)];
        let code = "function f(x){return x+x}".to_string();
        let js = CodeBlock {
            id,
            code_block_type,
            block_type,
            inputs,
            outputs,
            code,
        };
        let js_string: String = r#"
            {
                "id": "js_id",
                "block_type": "Code",
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
                "code": "function f(x){return x+x}"
            }
        "#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
        let expected = js_string.replace("functionf(x){returnx+x}", "function f(x){return x+x}");
        let result = serde_json::to_string(&js);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn js_json_deserialize() {
        let id = Id::new("js_id");
        let block_type = BlockType::Code;
        let code_block_type = CodeBlockType::Js;
        let inputs = vec![Input::new("input_id_1", DataType::Text)];
        let outputs = vec![Output::new("output_id_1", DataType::Text)];
        let code = "function f(x){return x+x}".to_string();
        let expected = CodeBlock {
            id,
            block_type,
            code_block_type,
            inputs,
            outputs,
            code,
        };
        let payload: String = r#"
            {
                "id": "js_id",
                "block_type": "Code",
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
                "code": "function f(x){return x+x}"
            }
        "#
        .to_string();
        let result = serde_json::from_str::<CodeBlock>(&payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
