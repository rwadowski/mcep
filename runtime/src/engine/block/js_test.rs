#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::time::Instant;
    use definition::block::{BlockType, Input, Output};
    use definition::block::js::{JsBlock as JsBlockDefinition};
    use definition::{DataType, Id};
    use crate::{DataFrame, InstanceId, Name, Origin};
    use crate::engine::applications::ApplicationId;
    use crate::engine::block::Block;
    use crate::engine::block::js::JsBlock;
    use crate::engine::{BlockId, Data};

    #[test]
    fn run_js_block() { ;
        let script = r#"
            function logic(input) {
                return {
                    "z": { "Text": input.x["Text"] + " " + input.y["Text"] }
                };
            }
        "#.to_string();
        let application_id = ApplicationId("application_id".to_string());
        let id = Id::new("definition_id");
        let block_id = BlockId::new(&application_id, &id);
        let x_input = "x".to_string();
        let y_input = "y".to_string();
        let output = "output".to_string();
        let definition = JsBlockDefinition {
            id,
            block_type: BlockType::Js,
            inputs: vec!(
                Input {
                    name: x_input,
                    data_type: DataType::Text,
                },
                Input {
                    name: y_input,
                    data_type: DataType::Text,
                }
            ),
            outputs: vec!(
                Output {
                    name: output,
                    data_type: DataType::Text,
                }
            ),
            code: script,
        };
        let input_x_frame_name = Name::from("x".to_string());
        let input_y_frame_name = Name::from("y".to_string());
        let output_frame_name = Name::from("z".to_string());
        let mut output_mappings: HashMap<Name, Name> = HashMap::new();
        output_mappings.insert(output_frame_name.clone(), output_frame_name.clone());
        let mut block = JsBlock::new(
            &application_id,
            definition,
        );
        let input_x = DataFrame::new(
            Origin::from(InstanceId("src".to_string())),
            Instant::now(),
            input_x_frame_name.clone(),
            Data::Text("hello".to_string()),
        );
        let input_y = DataFrame::new(
            Origin::from(InstanceId("src".to_string())),
            Instant::now(),
            input_y_frame_name.clone(),
            Data::Text("world".to_string()),
        );
        let mut result = block.run(input_x);
        result = block.run(input_y);
        let expected = DataFrame::new(
            Origin::from(block.id),
            Instant::now(),
            output_frame_name.clone(),
            Data::Text("hello world".to_string()),
        );
        assert_eq!(result.is_ok(), true);
        let res = result.unwrap();
        assert_eq!(res.len(), 1);
        let df = res.get(0).unwrap();
        assert_eq!(df.origin, expected.origin);
        assert_eq!(df.payload, expected.payload);
    }

    #[test]
    fn sandbox_test() {
        let v = Data::Text("txt".to_string());
        let result = serde_json::to_string(&v);
        println!("{}", result.unwrap())
    }
}