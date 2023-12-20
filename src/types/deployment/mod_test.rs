#[cfg(test)]
mod tests {
    use crate::definition::DataType;
    use crate::deployment::connection::junction::BlockJunction;
    use crate::deployment::connection::BlockConnection;
    use crate::deployment::sink::{Sink, SinkId};
    use crate::deployment::source::{Source, SourceId};
    use crate::deployment::{BlockId, DeployedBlock, Deployment, DeploymentId};

    #[test]
    fn test_serialize_body() {
        let id: DeploymentId = 0;
        let name = "title".to_string();
        let version = "1.0.0".to_string();
        let mut sources: Vec<Source> = Vec::new();
        sources.push(Source {
            id: SourceId::from("source_1"),
            data_type: DataType::Text,
        });
        let mut sinks: Vec<Sink> = Vec::new();
        sinks.push(Sink {
            id: SinkId::from("sink_1"),
            data_type: DataType::Text,
        });
        let blocks: Vec<DeployedBlock> = Vec::from([DeployedBlock {
            definition_id: 1,
            id: 1,
        }]);
        let body = Deployment {
            id,
            name,
            version,
            connections: connections(),
            sources,
            sinks,
            blocks,
        };
        let expected: String = r#"{
          "id": 0,
          "name": "title",
          "version": "1.0.0",
          "connections": [
            {
              "from": {
                "source": "source_1",
                "data_type": "Text"
              },
              "to": {
                "block": "1.1",
                "data_type": "Text"
              }
            },
            {
              "from": {
                "block": "1.1",
                "data_type": "Text"
              },
              "to": {
                "sink": "sink_1",
                "data_type": "Text"
              }
            }
          ],
          "sources": [
            {
              "id": "source_1",
              "data_type": "Text"
            }
          ],
          "sinks": [
            {
              "id": "sink_1",
              "data_type": "Text"
            }
          ],
          "blocks": [
            {
                "definition_id": 1,
                "id": 1
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
                "source": "source_1",
                "data_type": "Text"
              },
              "to": {
                "block": "1.1",
                "data_type": "Text"
              }
            },
            {
              "from": {
                "block": "1.1",
                "data_type": "Text"
              },
              "to": {
                "sink": "sink_1",
                "data_type": "Text"
              }
            }
          ],
          "sources": [
            {
              "id": "source_1",
              "data_type": "Text"
            }
          ],
          "sinks": [
            {
              "id": "sink_1",
              "data_type": "Text"
            }
          ],
          "description": "description",
          "help": "help",
          "blocks": [
            {
                "definition_id": 1,
                "id": 1
            }
          ]
        }"#;
        let id: DeploymentId = 0;
        let name = "title".to_string();
        let version = "1.0.0".to_string();
        let mut sources: Vec<Source> = Vec::new();
        sources.push(Source {
            id: SourceId::from("source_1"),
            data_type: DataType::Text,
        });
        let mut sinks: Vec<Sink> = Vec::new();
        sinks.push(Sink {
            id: SinkId::from("sink_1"),
            data_type: DataType::Text,
        });
        let blocks = Vec::from([DeployedBlock {
            definition_id: 1,
            id: 1,
        }]);
        let expected = Deployment {
            id,
            name,
            version,
            connections: connections(),
            sources,
            sinks,
            blocks,
        };

        let result = serde_json::from_str::<Deployment>(payload).unwrap();
        assert_eq!(result.id, expected.id);
        assert_eq!(result.connections, expected.connections);
        assert_eq!(result.name, expected.name);
        assert_eq!(result.version, expected.version);
        assert_eq!(result.sinks, expected.sinks);
        assert_eq!(result.sources, expected.sources);
        assert_eq!(result.blocks, expected.blocks);
    }

    fn connections() -> Vec<BlockConnection> {
        let mut connections: Vec<BlockConnection> = Vec::new();
        connections.push(BlockConnection {
            from: BlockJunction::from_source_id(SourceId::from("source_1"), DataType::Text),
            to: BlockJunction::from_block_id(BlockId::try_from("1.1").unwrap(), DataType::Text),
        });
        connections.push(BlockConnection {
            from: BlockJunction::from_block_id(BlockId::try_from("1.1").unwrap(), DataType::Text),
            to: BlockJunction::from_sink_id(SinkId::from("sink_1"), DataType::Text),
        });
        connections
    }
}
