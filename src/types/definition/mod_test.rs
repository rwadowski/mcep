#[cfg(test)]
mod tests {
    use crate::types::definition::DataType;
    use test_case::test_case;

    #[test_case(DataType::Boolean, "\"Boolean\""; "serialization of boolean is correct")]
    #[test_case(DataType::UnsignedInt, "\"UnsignedInt\""; "serialization of unsigned int is correct")]
    #[test_case(DataType::SignedInt, "\"SignedInt\""; "serialization of signed int is correct")]
    #[test_case(DataType::FloatType, "\"FloatType\""; "serialization of float is correct")]
    #[test_case(DataType::Text, "\"Text\""; "serialization of text is correct")]
    // #[test_case(DataType::Array(Box::from(DataType::Text)), "{\"Array\":\"Text\"}"; "serialization of array is correct")]
    // #[test_case(DataType::Map(Box::from(DataType::Text), Box::from(DataType::Text)), "{\"Map\":[\"Text\",\"Text\"]}"; "serialization of map is correct")]
    fn test_data_type_serialization(dt: DataType, expected: &str) {
        let result = serde_json::to_string(&dt);
        assert_eq!(true, result.is_ok());
        assert_eq!(expected.to_string(), result.unwrap());
    }

    #[test_case("\"Boolean\"", DataType::Boolean; "deserialization of boolean is correct")]
    #[test_case("\"UnsignedInt\"", DataType::UnsignedInt; "deserialization of unsigned int is correct")]
    #[test_case("\"SignedInt\"", DataType::SignedInt; "deserialization of signed int is correct")]
    #[test_case("\"FloatType\"", DataType::FloatType; "deserialization of float is correct")]
    #[test_case("\"Text\"", DataType::Text; "deserialization of text is correct")]
    // #[test_case("{\"Array\":\"Text\"}", DataType::Array(Box::from(DataType::Text)); "deserialization of array is correct")]
    // #[test_case("{\"Map\":[\"Text\",\"Text\"]}", DataType::Map(Box::from(DataType::Text), Box::from(DataType::Text)); "deserialization of map is correct")]
    fn test_data_type_deserialization(data: &str, expected: DataType) {
        let result = serde_json::from_str::<DataType>(data);
        assert_eq!(true, result.is_ok());
        assert_eq!(expected, result.unwrap());
    }
}
