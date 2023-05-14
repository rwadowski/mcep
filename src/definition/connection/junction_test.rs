#[cfg(test)]
mod junction_test {
    use crate::definition::connection::junction::Junction;
    use crate::definition::error::DefinitionError;
    use crate::definition::{DataType, Id};

    #[test]
    fn create_junction_id_success() {
        let input = "block_id.input_id";
        let junction_id = Junction::new(input, DataType::Text);
        let expected = Junction {
            parent: Id::new("block_id"),
            id: Id::new("input_id"),
            data_type: DataType::Text,
        };

        assert_eq!(junction_id.unwrap(), expected)
    }

    //TODO - test case for longer chains like 'block_id.block_id.input_id'
    #[test]
    fn create_junction_id_failure() {
        let input = "input_id";
        let junction_id = Junction::new(input, DataType::Text);
        let expected = DefinitionError::IncorrectJunctionString;

        assert_eq!(junction_id.is_err(), true);
        assert_eq!(junction_id.err().unwrap(), expected);
    }

    #[test]
    fn junction_json_serialization() {
        let payload = r#"
            {
                "parent": "parent_id",
                "id": "id",
                "data_type": "Text"
            }"#;
        let expected = Junction {
            parent: Id("parent_id".to_string()),
            id: Id("id".to_string()),
            data_type: DataType::Text,
        };
        let result = serde_json::from_str::<Junction>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn junction_json_deserialization() {
        let junction = Junction {
            parent: Id("parent_id".to_string()),
            id: Id("id".to_string()),
            data_type: DataType::Text,
        };
        let expected: String = r#"
            {
                "parent": "parent_id",
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
}