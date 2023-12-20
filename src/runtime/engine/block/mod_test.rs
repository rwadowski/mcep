#[cfg(test)]
mod test {
    use crate::runtime::engine::block;
    use crate::types::definition::block::code::CodeBlock as CodeBlockDefinition;
    use crate::types::definition::block::{BlockType, CodeBlockType, Input, Output};
    use crate::types::definition::DataType;
    use crate::types::deployment::{BlockInstanceId, DeploymentId};

    #[test]
    fn create_from_correct_definition() {
        let deployment_id: DeploymentId = 1;
        let block_id: BlockInstanceId = 1;
        let block_type = BlockType::Code;
        let input_1_name = "input_1_name".to_string();
        let input_1_data_type = DataType::Text;
        let output_1_name = "output_1_name".to_string();
        let output_1_data_type = DataType::Text;
        let lang = CodeBlockType::Python;
        let inputs = vec![Input {
            name: input_1_name,
            data_type: input_1_data_type,
        }];
        let outputs = vec![Output {
            name: output_1_name,
            data_type: output_1_data_type,
        }];
        let code = "function logic(x) { return x + '_' + x; }".to_string();
        let definition = CodeBlockDefinition {
            code_block_type: lang,
            block_type,
            inputs,
            outputs,
            code,
        };
        let result = block::new_block(Box::new(definition), deployment_id, block_id);
        assert_eq!(result.is_ok(), true);
    }
}
