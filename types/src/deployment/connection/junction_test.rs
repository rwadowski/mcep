#[cfg(test)]
mod junction_test {
    use crate::definition::DataType;
    use crate::deployment::connection::junction::BlockJunction;
    use crate::deployment::BlockId;

    #[test]
    fn create_block_junction_id_success() {
        let input = BlockId::try_from("1.2.3").unwrap();
        let junction = BlockJunction::from_block_id(input, DataType::Text);
        let expected = BlockJunction {
            block: Some(BlockId::try_from("1.2.3").unwrap()),
            sink: None,
            source: None,
            data_type: DataType::Text,
        };

        assert_eq!(junction, expected)
    }

    #[test]
    fn block_junction_json_deserialization() {
        let payload = r#"
            {
                "block": "1.2.3",
                "data_type": "Text"
            }"#;
        let expected = BlockJunction {
            block: Some(BlockId::try_from("1.2.3").unwrap()),
            sink: None,
            source: None,
            data_type: DataType::Text,
        };
        let result = serde_json::from_str::<BlockJunction>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn block_junction_json_serialization() {
        let junction = BlockJunction {
            block: Some(BlockId::try_from("1.2.3").unwrap()),
            sink: None,
            source: None,
            data_type: DataType::Text,
        };
        let expected: String = r#"
            {
                "block": "1.2.3",
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
