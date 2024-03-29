#[cfg(test)]
mod test {
    use crate::types::definition::DataType;
    use crate::types::deployment::sink::{Sink, SinkId};

    #[test]
    fn sink_json_deserialization() {
        let id = SinkId::from("sink_id");
        let dt = DataType::Text;
        let payload = r#"
            {
                "id": "sink_id",
                "data_type": "Text"
            }"#;
        let expected = Sink::new(id, dt);
        let result = serde_json::from_str::<Sink>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn sink_json_serialization() {
        let id = SinkId::from("sink_id");
        let dt = DataType::Text;
        let sink = Sink::new(id, dt);
        let expected: String = r#"
            {
                "id": "sink_id",
                "data_type": "Text"
            }
        "#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
        let result = serde_json::to_string(&sink);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}
