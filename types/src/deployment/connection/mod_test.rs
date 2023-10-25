#[cfg(test)]
mod test {
    use crate::definition::DataType;
    use crate::definition::error::DefinitionError;
    use crate::deployment::connection::Connection;
    use crate::deployment::connection::junction::Junction;

    #[test]
    fn create_connection_success() {
        let out = "block_1.output_1";
        let inp = "block_2.input_1";
        let output = Junction::new(out, DataType::Text).unwrap();
        let input = Junction::new(inp, DataType::Text).unwrap();
        let expected = Connection {
            from: Junction::new(out, DataType::Text).unwrap(),
            to: Junction::new(inp, DataType::Text).unwrap(),
        };

        let connection = Connection::new(output, input);

        assert_eq!(connection.unwrap(), expected);
    }

    #[test]
    fn create_connection_not_matching_types() {
        let out = "block_1.output_1";
        let inp = "block_2.input_1";
        let output = Junction::new(out, DataType::Text).unwrap();
        let input = Junction::new(inp, DataType::Boolean).unwrap();
        let expected = DefinitionError::IncorrectJunctionDataTypes;

        let connection = Connection::new(output, input);

        assert_eq!(connection.err().unwrap(), expected);
    }

    #[test]
    fn connection_json_serialization() {
        let from = Junction::new("p1.j1", DataType::Text).unwrap();
        let to = Junction::new("p2.j2", DataType::Text).unwrap();
        let connection = Connection::new(from, to).unwrap();
        let expected: String = r#"
            {
                "from": {
                    "block": "p1",
                    "id": "j1",
                    "data_type": "Text"
                },
                "to": {
                    "block": "p2",
                    "id": "j2",
                    "data_type": "Text"
                }
            }"#.chars().filter(|c| !c.is_whitespace()).collect();
        let result = serde_json::to_string(&connection);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn connection_json_deserialization() {
        let from = Junction::new("p1.j1", DataType::Text).unwrap();
        let to = Junction::new("p2.j2", DataType::Text).unwrap();
        let expected = Connection::new(from, to).unwrap();
        let payload = r#"
            {
                "from": {
                    "block": "p1",
                    "id": "j1",
                    "data_type": "Text"
                },
                "to": {
                    "block": "p2",
                    "id": "j2",
                    "data_type": "Text"
                }
            }"#;
        let result = serde_json::from_str::<Connection>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
