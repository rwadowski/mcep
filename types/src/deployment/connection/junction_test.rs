#[cfg(test)]
mod junction_test {
    use crate::definition::{DataType, Id};
    use crate::definition::error::DefinitionError;
    use crate::deployment::BlockId;
    use crate::deployment::connection::junction::{BlockJunction, DefinitionJunction};

    #[test]
    fn create_definition_junction_id_success() {
        let input = "1.input_id";
        let junction = DefinitionJunction::new(input, DataType::Text);
        let expected = DefinitionJunction {
            block: 1,
            id: Id::new("input_id"),
            data_type: DataType::Text,
        };

        assert_eq!(junction.unwrap(), expected)
    }

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

    //TODO - test case for longer chains like 'block_id.block_id.input_id'
    #[test]
    fn create_definition_junction_failure() {
        let input = "input_id";
        let junction = DefinitionJunction::new(input, DataType::Text);
        let expected = DefinitionError::IncorrectJunctionString;

        assert_eq!(junction.is_err(), true);
        assert_eq!(junction.err().unwrap(), expected);
    }

    #[test]
    fn create_definition_junction_type_failure() {
        let input = "block_id.input_id";
        let junction = DefinitionJunction::new(input, DataType::Text);
        let expected = DefinitionError::IncorrectJunctionString;

        assert_eq!(junction.is_err(), true);
        assert_eq!(junction.err().unwrap(), expected);
    }

    #[test]
    fn definition_junction_json_deserialization() {
        let payload = r#"
            {
                "block": 1,
                "id": "id",
                "data_type": "Text"
            }"#;
        let expected = DefinitionJunction {
            block: 1,
            id: Id::new("id"),
            data_type: DataType::Text,
        };
        let result = serde_json::from_str::<DefinitionJunction>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected)
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
    fn definition_junction_json_serialization() {
        let junction = DefinitionJunction {
            block: 1,
            id: Id::new("id"),
            data_type: DataType::Text,
        };
        let expected: String = r#"
            {
                "block": 1,
                "id": "id",
                "data_type": "Text"
            }"#
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        let result = serde_json::to_string(&junction);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
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