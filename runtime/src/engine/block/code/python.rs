use std::collections::BTreeMap;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use crate::engine::Data;

struct PythonBlock {
    pub code: String
}


impl PythonBlock {

    //TODO - initialize python during creation of struct
    fn run_python_code(&self, input: BTreeMap<String, Data>) -> () {
        Python::with_gil(|py| {
            let f: Py<PyAny> = PyModule::from_code(py, self.code.as_str())?
                .getattr("logic")?
                .into();
            let values = input.values().into_iter();
            let args = PyTuple::new(py, values);
            let result = f.call1(py, args);
                ()
        })
    }
}

fn