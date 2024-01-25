#[cfg(test)]
mod test {
    use crate::types::definition::block::code::github::Github;
    use crate::types::definition::block::code::github::Source;
    use crate::types::definition::block::{Block, BlockType, Input, Output};
    use crate::types::definition::DataType;

    #[test]
    fn github_json_serialize() {
        let inputs = vec![Input::new("input_id_1", DataType::Text)];
        let outputs = vec![Output::new("output_id_1", DataType::Text)];
        let owner = "owner".to_string();
        let repository = "repository".to_string();
        let token = "token".to_string();
        let path = "path".to_string();
        let source = Source {
            owner,
            repository,
            token,
            path,
        };
        let block: Box<dyn Block> = Box::new(Github {
            inputs,
            outputs,
            source,
        });
        let expected: String = r#"
            {
                "type": "Github",
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
                "source": {
                    "owner": "owner",
                    "repository": "repository",
                    "token": "token",
                    "path": "path"
                }
            }
        "#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

        let result = serde_json::to_string(block.as_ref());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn github_json_deserialize() {
        let block_type = BlockType::Github;
        let inputs = vec![Input::new("input_id_1", DataType::Text)];
        let outputs = vec![Output::new("output_id_1", DataType::Text)];
        let owner = "owner".to_string();
        let repository = "repository".to_string();
        let token = "token".to_string();
        let path = "path".to_string();
        let source = Source {
            owner,
            repository,
            token,
            path,
        };
        let expected = Github {
            inputs,
            outputs,
            source,
        };
        let payload: String = r#"
            {
                "type": "Github",
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
                "source": {
                    "owner": "owner",
                    "repository": "repository",
                    "token": "token",
                    "path": "path"
                }
            }
        "#
        .to_string();

        let result = serde_json::from_str::<Github>(&payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
