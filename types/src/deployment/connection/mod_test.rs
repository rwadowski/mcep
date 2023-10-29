#[cfg(test)]
mod test {
    use crate::definition::DataType;
    use crate::definition::error::DefinitionError;
    use crate::deployment::BlockId;
    use crate::deployment::connection::{BlockConnection, DefinitionConnection};
    use crate::deployment::connection::junction::{BlockJunction, DefinitionJunction};

    #[test]
    fn create_definition_connection_success() {
        let out = "1.output_1";
        let inp = "2.input_1";
        let output = DefinitionJunction::new(out, DataType::Text).unwrap();
        let input = DefinitionJunction::new(inp, DataType::Text).unwrap();
        let expected = DefinitionConnection {
            from: DefinitionJunction::new(out, DataType::Text).unwrap(),
            to: DefinitionJunction::new(inp, DataType::Text).unwrap(),
        };

        let connection = DefinitionConnection::new(output, input);

        assert_eq!(connection.unwrap(), expected);
    }

    #[test]
    fn create_block_connection_success() {
        let out = BlockId::try_from("block_1.output_1").unwrap();
        let inp = BlockId::try_from("block_2.input_1").unwrap();
        let output = BlockJunction::new(out.clone(), DataType::Text);
        let input = BlockJunction::new(inp.clone(), DataType::Text);
        let expected = BlockConnection {
            from: BlockJunction::new(out, DataType::Text),
            to: BlockJunction::new(inp, DataType::Text),
        };

        let connection = BlockConnection::new(output, input);

        assert_eq!(connection.unwrap(), expected);
    }

    #[test]
    fn create_definition_connection_not_matching_types() {
        let out = "1.output_1";
        let inp = "2.input_1";
        let output = DefinitionJunction::new(out, DataType::Text).unwrap();
        let input = DefinitionJunction::new(inp, DataType::Boolean).unwrap();
        let expected = DefinitionError::IncorrectJunctionDataTypes;

        let connection = DefinitionConnection::new(output, input);

        assert_eq!(connection.err().unwrap(), expected);
    }


    #[test]
    fn create_block_connection_not_matching_types() {
        let out = BlockId::try_from("block_1.output_1").unwrap();
        let inp = BlockId::try_from("block_2.input_1").unwrap();
        let output = BlockJunction::new(out, DataType::Text);
        let input = BlockJunction::new(inp, DataType::Boolean);
        let expected = DefinitionError::IncorrectJunctionDataTypes;

        let connection = BlockConnection::new(output, input);

        assert_eq!(connection.err().unwrap(), expected);
    }
    #[test]
    fn definition_connection_json_serialization() {
        let from = DefinitionJunction::new("1.j1", DataType::Text).unwrap();
        let to = DefinitionJunction::new("2.j2", DataType::Text).unwrap();
        let connection = DefinitionConnection::new(from, to).unwrap();
        let expected: String = r#"
            {
                "from": {
                    "block": 1,
                    "id": "j1",
                    "data_type": "Text"
                },
                "to": {
                    "block": 2,
                    "id": "j2",
                    "data_type": "Text"
                }
            }"#.chars().filter(|c| !c.is_whitespace()).collect();
        let result = serde_json::to_string(&connection);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn block_connection_json_serialization() {
        let from = BlockJunction::new(BlockId::try_from("block_1.j1").unwrap(), DataType::Text);
        let to = BlockJunction::new(BlockId::try_from("block_2.j2").unwrap(), DataType::Text);
        let connection = BlockConnection::new(from, to).unwrap();
        let expected: String = r#"
            {
                "from": {
                    "block": "block_1.j1",
                    "data_type": "Text"
                },
                "to": {
                    "block": "block_2.j2",
                    "data_type": "Text"
                }
            }"#.chars().filter(|c| !c.is_whitespace()).collect();
        let result = serde_json::to_string(&connection);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn definition_connection_json_deserialization() {
        let from = DefinitionJunction::new("1.j1", DataType::Text).unwrap();
        let to = DefinitionJunction::new("2.j2", DataType::Text).unwrap();
        let expected = DefinitionConnection::new(from, to).unwrap();
        let payload = r#"
            {
                "from": {
                    "block": 1,
                    "id": "j1",
                    "data_type": "Text"
                },
                "to": {
                    "block": 2,
                    "id": "j2",
                    "data_type": "Text"
                }
            }"#;
        let result = serde_json::from_str::<DefinitionConnection>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn block_connection_json_deserialization() {
        let from = BlockJunction::new(BlockId::try_from("block_1.j1").unwrap(), DataType::Text);
        let to = BlockJunction::new(BlockId::try_from("block_2.j2").unwrap(), DataType::Text);
        let expected = BlockConnection::new(from, to).unwrap();
        let payload = r#"
            {
                "from": {
                    "block": "block_1.j1",
                    "data_type": "Text"
                },
                "to": {
                    "block": "block_2.j2",
                    "data_type": "Text"
                }
            }"#;
        let result = serde_json::from_str::<BlockConnection>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
