#[cfg(test)]
mod python {
    use crate::engine::Data;
    use pyo3::prelude::*;
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
            let expected: Vec<Data> =
                vec![Data::Text(element_1.clone()), Data::Text(element_2.clone())];
            let result = expected.to_object(py);
            let value: PyResult<Vec<Data>> = result.extract(py);
            assert_eq!(true, value.is_ok());
            assert_eq!(expected, value.unwrap());
        })
    }
}
