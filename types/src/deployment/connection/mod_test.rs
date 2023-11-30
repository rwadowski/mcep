#[cfg(test)]
mod test {
    use crate::definition::error::DefinitionError;
    use crate::definition::DataType;
    use crate::deployment::connection::junction::BlockJunction;
    use crate::deployment::connection::BlockConnection;
    use crate::deployment::BlockId;

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
            }"#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
        let result = serde_json::to_string(&connection);
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
