#[cfg(test)]
mod tests {
    use services::deployment::create::NewDeployment;
    use types::definition::{DataType, DefinitionId};
    use types::deployment::connection::junction::BlockJunction;
    use types::deployment::connection::BlockConnection;
    use types::deployment::sink::{Sink, SinkId};
    use types::deployment::source::{Source, SourceId};
    use types::deployment::{BlockId, BlockInstanceId, DeployedBlock};

    #[test]
    fn test_new_deployment_deserialize_body() {
        let payload = r#"{
            "name": "deployment test",
            "version": "1.0.0",
            "sources": [
                {
                    "id": "mcep-kafka-source",
                    "data_type": "Text"
                }
            ],
            "sinks": [
                {
                    "id": "mcep-kafka-sink",
                    "data_type": "Text"
                }
            ],
            "blocks": [
                {
                    "definition_id": 1,
                    "id": 1
                }
            ],
            "connections": [
                {
                    "from": {
                        "source": "mcep-kafka-source",
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
                        "sink": "mcep-kafka-sink",
                        "data_type": "Text"
                    }
                }
            ]
        }"#;
        let name = "deployment test".to_string();
        let version = "1.0.0".to_string();
        let source_id = SourceId::from("mcep-kafka-source");
        let sources = Vec::from([Source::new(source_id, DataType::Text)]);
        let sink_id = SinkId::from("mcep-kafka-sink");
        let sinks = Vec::from([Sink::new(sink_id, DataType::Text)]);
        let definition_id: DefinitionId = 1;
        let block_id: BlockInstanceId = 1;
        let blocks = Vec::from([DeployedBlock::new(definition_id, block_id)]);
        let expected = NewDeployment {
            name,
            version,
            connections: connections(),
            sources,
            sinks,
            blocks,
        };

        let result = serde_json::from_str::<NewDeployment>(payload).unwrap();
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
            from: BlockJunction::from_source_id(
                SourceId::from("mcep-kafka-source"),
                DataType::Text,
            ),
            to: BlockJunction::from_block_id(BlockId::try_from("1.1").unwrap(), DataType::Text),
        });
        connections.push(BlockConnection {
            from: BlockJunction::from_block_id(BlockId::try_from("1.1").unwrap(), DataType::Text),
            to: BlockJunction::from_sink_id(SinkId::from("mcep-kafka-sink"), DataType::Text),
        });
        connections
    }
}
