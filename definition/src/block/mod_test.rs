#[cfg(test)]
mod test {
    use crate::{DataType, Id};
    use crate::block::{Input, Output};

    #[test]
    fn id_json_serialize() {
        let id: Id = Id("test_id".to_string());
        let expected = "\"test_id\"".to_string();
        let result = serde_json::to_string(&id);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn id_json_deserialize() {
        let string = "\"test_id\"".to_string();
        let expected = Id("test_id".to_string());
        let result = serde_json::from_str::<Id>(string.as_str());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn input_json_serialize() {
        let name = "input_name";
        let dt = DataType::Text;
        let input = Input::new(name, dt);
        let expected: String = r#"
            {
                "name": "input_name",
                "data_type": "Text"
            }"#.chars().filter(|c| !c.is_whitespace()).collect();
        let result = serde_json::to_string(&input);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn input_json_deserialize() {
        let name = "input_name";
        let dt = DataType::Text;
        let payload = r#"
            {
                "name": "input_name",
                "data_type": "Text"
            }
        "#;
        let expected = Input::new(name, dt);
        let result = serde_json::from_str::<Input>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn output_json_serialize() {
        let name = "output_name";
        let dt = DataType::Text;
        let output = Output::new(name, dt);
        let expected: String = r#"
            {
                "name": "output_name",
                "data_type": "Text"
            }"#.chars().filter(|c| !c.is_whitespace()).collect();
        let result = serde_json::to_string(&output);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn output_json_deserialize() {
        let name = "output_name";
        let dt = DataType::Text;
        let payload = r#"
            {
                "name": "output_name",
                "data_type": "Text"
            }"#;
        let expected = Output::new(name, dt);
        let result = serde_json::from_str::<Output>(payload);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected);
    }
}