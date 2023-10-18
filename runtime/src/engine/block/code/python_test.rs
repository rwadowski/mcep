#[cfg(test)]
mod python {
    use std::collections::HashMap;
    use std::time::Instant;
    use pyo3::prelude::*;
    use definition::block::{BlockType, CodeBlockType, Input, Output};
    use definition::block::code::{CodeBlock as CodeBlockDefinition};
    use definition::{DataType, Id};
    use crate::{DataFrame, InstanceId, Name, Origin};
    use crate::engine::applications::ApplicationId;
    use crate::engine::block::Block;
    use crate::engine::block::code::CodeBlock;
    use crate::engine::Data;
    #[test]
    fn run_python_block() {
        let script = "def logic(v):
    r = {
        \"z\": v[\"x\"] + \" \" + v[\"y\"]
    }
    return r".to_string();
        let application_id = ApplicationId("application_id".to_string());
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
        let mut block = CodeBlock::new(
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
    fn bool_conversion() {
        Python::with_gil(|py| {
            let expected = true;
            let data = Data::Boolean(expected);
            let result = data.to_object(py);
            let value: PyResult<bool> = result.extract(py);
            assert_eq!(true, value.is_ok());
            assert_eq!(expected, value.unwrap());
        })
    }

    #[test]
    fn unsigned_int_conversion() {
        Python::with_gil(|py| {
            let expected: u64 = 1000;
            let data = Data::UnsignedInt(expected);
            let result = data.to_object(py);
            let value: PyResult<u64> = result.extract(py);
            assert_eq!(true, value.is_ok());
            assert_eq!(expected, value.unwrap());
        })
    }

    #[test]
    fn signed_int_conversion() {
        Python::with_gil(|py| {
            let expected: i64 = 300;
            let data = Data::SignedInt(expected);
            let result = data.to_object(py);
            let value: PyResult<i64> = result.extract(py);
            assert_eq!(true, value.is_ok());
            assert_eq!(expected, value.unwrap());
        })
    }

    #[test]
    fn text_conversion() {
        Python::with_gil(|py| {
            let expected: String = "text".to_string();
            let data = Data::Text(expected.clone());
            let result = data.to_object(py);
            let value: PyResult<String> = result.extract(py);
            assert_eq!(true, value.is_ok());
            assert_eq!(expected, value.unwrap());
        })
    }

    #[test]
    fn array_conversion() {
        Python::with_gil(|py| {
            let element_1 = "element_1".to_string();
            let element_2 = "element_2".to_string();
            let expected: Vec<Data> = vec!(
                Data::Text(element_1.clone()),
                Data::Text(element_2.clone()),
            );
            let result = expected.to_object(py);
            let value: PyResult<Vec<Data>> = result.extract(py);
            assert_eq!(true, value.is_ok());
            assert_eq!(expected, value.unwrap());
        })
    }
}