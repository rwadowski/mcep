#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::time::Instant;
    use pyo3::prelude::*;
    use types::definition::block::{BlockType, CodeBlockType, Input, Output};
    use types::definition::block::code::{CodeBlock as CodeBlockDefinition};
    use types::definition::{DataType, Id};
    use types::deployment::{Deployment, DeploymentId};
    use crate::{DataFrame, InstanceId, Name, Origin};
    use crate::engine::block::Block;
    use crate::engine::block::code::CodeBlock;
    use crate::engine::Data;
    #[test]
    fn run_code_block() {
        let script = "def logic(v):
    r = {
        \"z\": v[\"x\"] + \" \" + v[\"y\"]
    }
    return r".to_string();
        let deployment_id: DeploymentId = 0;
        let id = Id::new("definition_id");
        let x_input = "x".to_string();
        let y_input = "y".to_string();
        let output = "z".to_string();
        let definition = CodeBlockDefinition {
            id,
            code_block_type: CodeBlockType::Python,
            block_type: BlockType::Code,
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
        println!("{:}", serde_json::to_string(&definition).unwrap());
        let mut block = CodeBlock::new(
            &deployment_id,
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
}