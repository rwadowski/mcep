#[cfg(test)]
mod python {
    use crate::runtime::engine::Data;
    use pyo3::prelude::*;

    #[test]
    fn bool_conversion() {
        Python::attach(|py| {
            let expected = true;
            let data = Data::Boolean(expected);
            let object = data.into_pyobject(py).unwrap();
            let value: PyResult<bool> = object.extract();
            assert_eq!(true, value.is_ok());
            assert_eq!(expected, value.unwrap());
        })
    }

    #[test]
    fn unsigned_int_conversion() {
        Python::attach(|py| {
            let expected: u64 = 1000;
            let data = Data::UnsignedInt(expected);
            let object = data.into_pyobject(py).unwrap();
            let value: PyResult<u64> = object.extract();
            assert_eq!(true, value.is_ok());
            assert_eq!(expected, value.unwrap());
        })
    }

    #[test]
    fn signed_int_conversion() {
        Python::attach(|py| {
            let expected: i64 = 300;
            let data = Data::SignedInt(expected);
            let object = data.into_pyobject(py).unwrap();
            let value: PyResult<i64> = object.extract();
            assert_eq!(true, value.is_ok());
            assert_eq!(expected, value.unwrap());
        })
    }

    #[test]
    fn text_conversion() {
        Python::attach(|py| {
            let expected: String = "text".to_string();
            let data = Data::Text(expected.clone());
            let object = data.into_pyobject(py).unwrap();
            let value: PyResult<String> = object.extract();
            assert_eq!(true, value.is_ok());
            assert_eq!(expected, value.unwrap());
        })
    }

    #[test]
    fn array_conversion() {
        Python::attach(|py| {
            let element_1 = "element_1".to_string();
            let element_2 = "element_2".to_string();
            let expected: Vec<Data> =
                vec![Data::Text(element_1.clone()), Data::Text(element_2.clone())];
            let object = expected.clone().into_pyobject(py).unwrap();
            let value: PyResult<Vec<Data>> = object.extract();
            assert_eq!(true, value.is_ok());
            assert_eq!(expected, value.unwrap());
        })
    }
}
