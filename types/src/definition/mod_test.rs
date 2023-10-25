#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::definition::{DataType, Id};
    use crate::definition::connection::Connection;
    use crate::definition::connection::junction::Junction;
    use crate::definition::connection::sink::Sink;
    use crate::definition::connection::source::Source;
    use crate::deployment::{Deployment, DeploymentId};

    #[test_case(DataType::Boolean, "\"Boolean\""; "serialization of boolean is correct")]
    #[test_case(DataType::UnsignedInt, "\"UnsignedInt\""; "serialization of unsigned int is correct")]
    #[test_case(DataType::SignedInt, "\"SignedInt\""; "serialization of signed int is correct")]
    #[test_case(DataType::FloatType, "\"FloatType\""; "serialization of float is correct")]
    #[test_case(DataType::Text, "\"Text\""; "serialization of text is correct")]
    #[test_case(DataType::Array(Box::from(DataType::Text)), "{\"Array\":\"Text\"}"; "serialization of array is correct")]
    #[test_case(DataType::Map(Box::from(DataType::Text), Box::from(DataType::Text)), "{\"Map\":[\"Text\",\"Text\"]}"; "serialization of map is correct")]
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
    #[test_case("{\"Array\":\"Text\"}", DataType::Array(Box::from(DataType::Text)); "deserialization of array is correct")]
    #[test_case("{\"Map\":[\"Text\",\"Text\"]}", DataType::Map(Box::from(DataType::Text), Box::from(DataType::Text)); "deserialization of map is correct")]
    fn test_data_type_deserialization(data: &str, expected: DataType) {
        let result = serde_json::from_str::<DataType>(data);
        assert_eq!(true, result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_serialize_body() {
        let id: DeploymentId = 0;
        let name = "title".to_string();
        let version = "1.0.0".to_string();
        let mut sources: Vec<Source> = Vec::new();
        sources.push(Source{
            id: Id::new("source_1_id"),
            data_type: DataType::Text
        });
        let mut sinks: Vec<Sink> = Vec::new();
        sinks.push(Sink {
            id: Id::new("sink_1_id"),
            data_type: DataType::Text
        });
        let mut connections: Vec<Connection> = Vec::new();
        connections.push(Connection {
            from: Junction::new("app_id.source_1_id", DataType::Text).unwrap(),
            to: Junction::new("js_1.input_1_id", DataType::Text).unwrap(),
        });
        connections.push(Connection {
            from: Junction::new("js_1.output_1_id", DataType::Text).unwrap(),
            to: Junction::new("app_id.sink_1_id", DataType::Text).unwrap()
        });
        let body = Deployment {
            id,
            name,
            version,
            connections,
            sources,
            sinks,
        };
        let expected: String =
            r#"{
          "id": 0,
          "name": "title",
          "version": "1.0.0",
          "connections": [
            {
              "from": {
                "block": "app_id",
                "id": "source_1_id",
                "data_type": "Text"
              },
              "to": {
                "block": "js_1",
                "id": "input_1_id",
                "data_type": "Text"
              }
            },
            {
              "from": {
                "block": "js_1",
                "id": "output_1_id",
                "data_type": "Text"
              },
              "to": {
                "block": "app_id",
                "id": "sink_1_id",
                "data_type": "Text"
              }
            }
          ],
          "sources": [
            {
              "id": "source_1_id",
              "data_type": "Text"
            }
          ],
          "sinks": [
            {
              "id": "sink_1_id",
              "data_type": "Text"
            }
          ]
        }"#.chars().filter(|c| !c.is_whitespace()).collect();
        let result = serde_json::to_string_pretty(&body);
        assert_eq!(result.is_ok(), true);
        let result_str: String = result.unwrap().chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(result_str, expected);
    }

    #[test]
    fn test_deserialize_body() {
        let payload = r#"{
          "id": 0,
          "name": "title",
          "version": "1.0.0",
          "connections": [
            {
              "from": {
                "block": "app_id",
                "id": "source_1_id",
                "data_type": "Text"
              },
              "to": {
                "block": "js_1",
                "id": "input_1_id",
                "data_type": "Text"
              }
            },
            {
              "from": {
                "block": "js_1",
                "id": "output_1_id",
                "data_type": "Text"
              },
              "to": {
                "block": "app_id",
                "id": "sink_1_id",
                "data_type": "Text"
              }
            }
          ],
          "sources": [
            {
              "id": "source_1_id",
              "data_type": "Text"
            }
          ],
          "sinks": [
            {
              "id": "sink_1_id",
              "data_type": "Text"
            }
          ],
          "description": "description",
          "help": "help"
        }"#;
        let id: DeploymentId = 0;
        let name = "title".to_string();
        let version = "1.0.0".to_string();
        let mut sources: Vec<Source> = Vec::new();
        sources.push(Source{
            id: Id::new("source_1_id"),
            data_type: DataType::Text
        });
        let mut sinks: Vec<Sink> = Vec::new();
        sinks.push(Sink {
            id: Id::new("sink_1_id"),
            data_type: DataType::Text
        });
        let mut connections: Vec<Connection> = Vec::new();
        connections.push(Connection {
            from: Junction::new("app_id.source_1_id", DataType::Text).unwrap(),
            to: Junction::new("js_1.input_1_id", DataType::Text).unwrap(),
        });
        connections.push(Connection {
            from: Junction::new("js_1.output_1_id", DataType::Text).unwrap(),
            to: Junction::new("app_id.sink_1_id", DataType::Text).unwrap()
        });
        let expected = Deployment {
            id,
            name,
            version,
            connections,
            sources,
            sinks,
        };

        let result = serde_json::from_str::<Deployment>(payload).unwrap();
        assert_eq!(result.id, expected.id);
        assert_eq!(result.connections, expected.connections);
        assert_eq!(result.name, expected.name);
        assert_eq!(result.version, expected.version);
        assert_eq!(result.sinks, expected.sinks);
        assert_eq!(result.sources, expected.sources);
    }
}
