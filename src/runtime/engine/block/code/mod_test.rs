#[cfg(test)]
mod test {
    use crate::runtime::engine::block::code::PythonCodeBlock;
    use crate::runtime::engine::block::Block;
    use crate::runtime::engine::Data;
    use crate::runtime::{DataFrame, Name, Origin};
    use crate::types::definition::block::code::CodeBlock as CodeBlockDefinition;
    use crate::types::definition::block::{BlockType, Input, Output};
    use crate::types::definition::DataType;
    use crate::types::deployment::{BlockId, BlockInstanceId, DeploymentId};
    use chrono::Utc;
    use std::collections::HashMap;

    #[test]
    fn run_code_block() {
        let script = "def logic(v):
    r = {
        \"z\": v[\"x\"] + \" \" + v[\"y\"]
    }
    return r"
            .to_string();
        let deployment_id: DeploymentId = 0;
        let x_input = "x".to_string();
        let y_input = "y".to_string();
        let output = "z".to_string();
        let definition = CodeBlockDefinition {
            inputs: vec![
                Input {
                    name: x_input,
                    data_type: DataType::Text,
                },
                Input {
                    name: y_input,
                    data_type: DataType::Text,
                },
            ],
            outputs: vec![Output {
                name: output,
                data_type: DataType::Text,
            }],
            source: script.clone(),
            dependencies: vec![],
        };
        let input_x_frame_name = Name::from("x");
        let input_y_frame_name = Name::from("y");
        let output_frame_name = Name::from("z");
        let mut output_mappings: HashMap<Name, Name> = HashMap::new();
        output_mappings.insert(output_frame_name.clone(), output_frame_name.clone());
        let block_id: BlockInstanceId = 1;
        let mut block =
            PythonCodeBlock::new(script.clone(), deployment_id, block_id, definition.inputs, vec![]);
        let input_x = DataFrame::new(
            Origin::from(BlockId::new(deployment_id, block_id)),
            Utc::now(),
            input_x_frame_name.clone(),
            Data::Text("hello".to_string()),
        );
        let input_y = DataFrame::new(
            Origin::from(BlockId::new(deployment_id, block_id)),
            Utc::now(),
            input_y_frame_name.clone(),
            Data::Text("world".to_string()),
        );
        let mut input: HashMap<Name, Data> = HashMap::new();
        input.insert(input_x.name, input_x.value);
        input.insert(input_y.name, input_y.value);
        let result = block.run(&input);
        let expected = DataFrame::new(
            Origin::from(block.id),
            Utc::now(),
            output_frame_name.clone(),
            Data::Text("hello world".to_string()),
        );
        assert_eq!(result.is_ok(), true);
        let res = result.unwrap();
        assert_eq!(res.len(), 1);
        let df = res.get(0).unwrap();
        assert_eq!(df.origin, expected.origin);
        assert_eq!(df.value, expected.value);
    }
}
