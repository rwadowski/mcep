#[cfg(test)]
mod test {
    use crate::definition::DataType;
    use crate::deployment::source::{Source, SourceId};

    #[test]
    fn source_json_serialization() {
        let id = SourceId::from("source_id");
        let dt = DataType::Text;
        let source = Source::new(id, dt);
        let expected: String = r#"
            {
                "id": "source_id",
                "data_type": "Text"
            }"#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
        let result = serde_json::to_string(&source);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn source_json_deserialization() {
        let id = SourceId::from("source_id");
        let dt = DataType::Text;
        let expected = Source::new(id, dt);
        let payload = r#"
            {
                "id": "source_id",
                "data_type": "Text"
            }"#;
        let result = serde_json::from_str::<Source>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
