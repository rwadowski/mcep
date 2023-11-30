#[cfg(test)]
mod junction_test {
    use crate::definition::DataType;
    use crate::deployment::connection::junction::BlockJunction;
    use crate::deployment::BlockId;

    #[test]
    fn create_block_junction_id_success() {
        let input = BlockId::try_from("block_1.input_id").unwrap();
        let junction = BlockJunction::new(input, DataType::Text);
        let expected = BlockJunction {
            block: BlockId::try_from("block_1.input_id").unwrap(),
            data_type: DataType::Text,
        };

        assert_eq!(junction, expected)
    }

    #[test]
    fn block_junction_json_deserialization() {
        let payload = r#"
            {
                "block": "block_1.input_1",
                "data_type": "Text"
            }"#;
        let expected = BlockJunction {
            block: BlockId::try_from("block_1.input_1").unwrap(),
            data_type: DataType::Text,
        };
        let result = serde_json::from_str::<BlockJunction>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn block_junction_json_serialization() {
        let junction = BlockJunction {
            block: BlockId::try_from("block_1.input_1").unwrap(),
            data_type: DataType::Text,
        };
        let expected: String = r#"
            {
                "block": "block_1.input_1",
                "data_type": "Text"
            }"#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
        let result = serde_json::to_string(&junction);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
