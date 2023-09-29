use std::collections::{BTreeMap, HashMap};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use crate::DataFrame;
use crate::engine::Data;

pub(crate) struct PythonBlock {
    pub code: String
}


impl PythonBlock {

    //TODO - initialize python during creation of struct
    pub fn run_python_code(&self, input: HashMap<String, Data>) -> PyResult<()> {
        Python::with_gil(|py| {
            let function: Py<PyAny> =  PyModule::from_code(py, self.code.as_str(), "", "")?
                .getattr("logic")?
                .into();
            let args = PyTuple::new(py, input);
            let result = function.call1(py, args)?;

            Ok(())
            // let f: Py<PyAny> = PyModule::from_code(py, self.code.as_str(), "", "")?
            //     .getattr("logic")?
            //     .into();
            // let values = input.values().into_iter();
            // let args = PyTuple::new(py, values);
            // let result = f.call1(py, args);
            //     ()
        })
    }

    // fn to_data_frame(object: PyObject) -> DataFrame {
    // }
}


impl ToPyObject for Data {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            Data::Boolean(v) => v.into_py(py),
            Data::UnsignedInt(v) => v.into_py(py),
            Data::SignedInt(v) => v.into_py(py),
            Data::Text(v) => v.into_py(py),
            _ => panic!("ups")
        }
    }
}