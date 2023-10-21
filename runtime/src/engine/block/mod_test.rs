#[cfg(test)]
mod test {
    use types::definition::block::{BlockType, CodeBlockType, Input, Output};
    use definition::block::code::{CodeBlock as CodeBlockDefinition};
    use definition::{DataType, Id};
    use crate::engine::applications::ApplicationId;
    use crate::engine::block;

    #[test]
    fn create_from_correct_definition() {
        let definition_id = Id::new("definition_id");
        let block_type = BlockType::Code;
        let input_1_name = "input_1_name".to_string();
        let input_1_data_type = DataType::Text;
        let output_1_name = "output_1_name".to_string();
        let output_1_data_type = DataType::Text;
        let lang = CodeBlockType::Python;
        let inputs = vec!(
            Input {
                name: input_1_name,
                data_type: input_1_data_type,
            }
        );
        let outputs = vec!(
            Output {
                name: output_1_name,
                data_type: output_1_data_type,
            }
        );
        let code = "function logic(x) { return x + '_' + x; }".to_string();
        let definition = CodeBlockDefinition {
            id: definition_id,
            code_block_type: lang,
            block_type,
            inputs,
            outputs,
            code,
        };
        let application_id = ApplicationId("application_id".to_string());
        let result = block::new_block(application_id, Box::new(definition));
        assert_eq!(result.is_ok(), true);
    }
}