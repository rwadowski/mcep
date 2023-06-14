#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::{Body, DataType, Id};
    use crate::block::{Block, BlockType, Input, Output};
    use crate::block::js::Js;
    use crate::connection::Connection;
    use crate::connection::junction::Junction;
    use crate::connection::sink::Sink;
    use crate::connection::source::Source;

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
        let block_id = Id::new("js_id");
        let block_type = BlockType::Js;
        let inputs = vec![Input::new("input_id_1", DataType::Text)];
        let outputs = vec![Output::new("output_id_1", DataType::Text)];
        let code = "function f(x){return x+x}".to_string();
        let js = Js {
            id: block_id,
            block_type,
            inputs,
            outputs,
            code
        };
        let id = 0.to_string();
        let title = "title".to_string();
        let version = "1.0.0".to_string();
        let description = "description".to_string();
        let help = "help".to_string();
        let mut js_block_inputs: Vec<Input> = Vec::new();
        js_block_inputs.push(
            Input::new("input_1_id", DataType::Text)
        );
        let mut js_block_outputs: Vec<Output> = Vec::new();
        js_block_outputs.push(
            Output::new("output_1_id", DataType::Text)
        );
        let mut blocks: Vec<Box<dyn Block>> = Vec::new();
        blocks.push(Box::new(js));
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
        let body = Body {
            id,
            title,
            version,
            blocks,
            connections,
            sources,
            sinks,
            description: Some(description),
            help: Some(help),
        };
        let expected: String =
        r#"{
          "id": "0",
          "title": "title",
          "version": "1.0.0",
          "blocks": [
            {
              "type": "Js",
              "id": "js_id",
              "block_type": "Js",
              "inputs": [
                {
                  "name": "input_id_1",
                  "data_type": "Text"
                }
              ],
              "outputs": [
                {
                  "name": "output_id_1",
                  "data_type": "Text"
                }
              ],
              "code": "function f(x){return x+x}"
            }
          ],
          "connections": [
            {
              "from": {
                "parent": "app_id",
                "id": "source_1_id",
                "data_type": "Text"
              },
              "to": {
                "parent": "js_1",
                "id": "input_1_id",
                "data_type": "Text"
              }
            },
            {
              "from": {
                "parent": "js_1",
                "id": "output_1_id",
                "data_type": "Text"
              },
              "to": {
                "parent": "app_id",
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
        }"#.chars().filter(|c| !c.is_whitespace()).collect();
        let result = serde_json::to_string_pretty(&body);
        assert_eq!(result.is_ok(), true);
        let result_str: String = result.unwrap().chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(result_str, expected);
    }

    #[test]
    fn test_deserialize_body() {
        let payload = r#"{
          "id": "0",
          "title": "title",
          "version": "1.0.0",
          "blocks": [
            {
              "type": "Js",
              "id": "js_id",
              "block_type": "Js",
              "inputs": [
                {
                  "name": "input_id_1",
                  "data_type": "Text"
                }
              ],
              "outputs": [
                {
                  "name": "output_id_1",
                  "data_type": "Text"
                }
              ],
              "code": "function f(x){return x+x}"
            }
          ],
          "connections": [
            {
              "from": {
                "parent": "app_id",
                "id": "source_1_id",
                "data_type": "Text"
              },
              "to": {
                "parent": "js_1",
                "id": "input_1_id",
                "data_type": "Text"
              }
            },
            {
              "from": {
                "parent": "js_1",
                "id": "output_1_id",
                "data_type": "Text"
              },
              "to": {
                "parent": "app_id",
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
        let block_id = Id::new("js_id");
        let block_type = BlockType::Js;
        let inputs = vec![Input::new("input_id_1", DataType::Text)];
        let outputs = vec![Output::new("output_id_1", DataType::Text)];
        let code = "function f(x){return x+x}".to_string();
        let js = Js {
            id: block_id,
            block_type,
            inputs,
            outputs,
            code
        };
        let id = 0.to_string();
        let title = "title".to_string();
        let version = "1.0.0".to_string();
        let description = "description".to_string();
        let help = "help".to_string();
        let mut js_block_inputs: Vec<Input> = Vec::new();
        js_block_inputs.push(
            Input::new("input_1_id", DataType::Text)
        );
        let mut js_block_outputs: Vec<Output> = Vec::new();
        js_block_outputs.push(
            Output::new("output_1_id", DataType::Text)
        );
        let mut blocks: Vec<Box<dyn Block>> = Vec::new();
        blocks.push(Box::new(js));
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
        let expected = Body {
            id,
            title,
            version,
            blocks,
            connections,
            sources,
            sinks,
            description: Some(description),
            help: Some(help),
        };

        let result = serde_json::from_str::<Body>(payload).unwrap();
        assert_eq!(result.id, expected.id);
        assert_eq!(result.connections, expected.connections);
        assert_eq!(result.description, expected.description);
        assert_eq!(result.title, expected.title);
        assert_eq!(result.help, expected.help);
        assert_eq!(result.version, expected.version);
        assert_eq!(result.sinks, expected.sinks);
        assert_eq!(result.sources, expected.sources);
    }
}
