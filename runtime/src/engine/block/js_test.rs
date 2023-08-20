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
    fn run_js_block() {
        let script = r#"
            function logic(input) {
                return {
                    z: input.y + '_' + input.x
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
        let input_frame_name = Name::from("input_frame_name".to_string());
        let output_frame_name = Name::from("output_frame_name".to_string());
        let mut output_mappings: HashMap<Name, Name> = HashMap::new();
        output_mappings.insert(output_frame_name.clone(), output_frame_name.clone());
        let mut block = JsBlock::new(
            &application_id,
            definition,
        );
        let code = "function(x) { return x + '_' + x; }".to_string();
        let input = DataFrame::new(
            Origin::from(InstanceId("src".to_string())),
            Instant::now(),
            input_frame_name.clone(),
            Data::Text("text".to_string()),
        );
        let result = block.run(input);
        let expected = DataFrame::new(
            Origin::from(block.id),
            Instant::now(),
            output_frame_name.clone(),
            Data::Text("text_text".to_string()),
        );
        assert_eq!(result.is_ok(), true);
        let res = result.unwrap();
        // assert_eq!(res.origin, expected.origin);
        // assert_eq!(res.payload, expected.payload);
    }
}