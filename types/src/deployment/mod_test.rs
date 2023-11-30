#[cfg(test)]
mod tests {
    use crate::definition::{DataType, Id};
    use crate::deployment::connection::junction::BlockJunction;
    use crate::deployment::connection::BlockConnection;
    use crate::deployment::sink::Sink;
    use crate::deployment::source::Source;
    use crate::deployment::{BlockId, Deployment, DeploymentId};

    #[test]
    fn test_serialize_body() {
        let id: DeploymentId = 0;
        let name = "title".to_string();
        let version = "1.0.0".to_string();
        let mut sources: Vec<Source> = Vec::new();
        sources.push(Source {
            id: Id::new("source_1_id"),
            data_type: DataType::Text,
        });
        let mut sinks: Vec<Sink> = Vec::new();
        sinks.push(Sink {
            id: Id::new("sink_1_id"),
            data_type: DataType::Text,
        });
        let body = Deployment {
            id,
            name,
            version,
            connections: connections(),
            sources,
            sinks,
        };
        let expected: String = r#"{
          "id": 0,
          "name": "title",
          "version": "1.0.0",
          "connections": [
            {
              "from": {
                "block": "1.source_1_id",
                "data_type": "Text"
              },
              "to": {
                "block": "2.input_1_id",
                "data_type": "Text"
              }
            },
            {
              "from": {
                "block": "1.output_1_id",
                "data_type": "Text"
              },
              "to": {
                "block": "2.sink_1_id",
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
        }"#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
        let result = serde_json::to_string_pretty(&body);
        assert_eq!(result.is_ok(), true);
        let result_str: String = result
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
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
                "block": "1.source_1_id",
                "data_type": "Text"
              },
              "to": {
                "block": "2.input_1_id",
                "data_type": "Text"
              }
            },
            {
              "from": {
                "block": "1.output_1_id",
                "data_type": "Text"
              },
              "to": {
                "block": "2.sink_1_id",
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
        sources.push(Source {
            id: Id::new("source_1_id"),
            data_type: DataType::Text,
        });
        let mut sinks: Vec<Sink> = Vec::new();
        sinks.push(Sink {
            id: Id::new("sink_1_id"),
            data_type: DataType::Text,
        });
        let expected = Deployment {
            id,
            name,
            version,
            connections: connections(),
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

    fn connections() -> Vec<BlockConnection> {
        let mut connections: Vec<BlockConnection> = Vec::new();
        connections.push(BlockConnection {
            from: BlockJunction::new(BlockId::try_from("1.source_1_id").unwrap(), DataType::Text),
            to: BlockJunction::new(BlockId::try_from("2.input_1_id").unwrap(), DataType::Text),
        });
        connections.push(BlockConnection {
            from: BlockJunction::new(BlockId::try_from("1.output_1_id").unwrap(), DataType::Text),
            to: BlockJunction::new(BlockId::try_from("2.sink_1_id").unwrap(), DataType::Text),
        });
        connections
    }
}
