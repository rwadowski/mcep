
use std::collections::HashMap;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyInt, PyLong, PyString, PyList, PyFloat};
use pyo3::types::IntoPyDict;
use crate::engine::Data;

pub(crate) struct PythonBlock {
    pub code: String
}

impl PythonBlock {
    pub fn run(&self, input: HashMap<String, Data>) -> Result<HashMap<String, Data>, String> {
        Python::with_gil(|py| {
            let function: Py<PyAny> = PyModule::from_code(py, self.code.as_str(), "", "")
                .map_err(|e| e.to_string())?
                .getattr("logic")
                .map_err(|e| e.to_string())?
                .into();
            let args = (input.into_py_dict(py),);
            let result = function.call(py, args, None);
            match result {
                Ok(object) => {
                    let map: HashMap<String, Data> = object.extract(py)
                        .map_err(|e| e.to_string())?;
                    Ok(map)
                },
                Err(e) => Err(e.to_string())
            }
        })
    }
}


impl ToPyObject for Data {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            Data::Boolean(v) => v.into_py(py),
            Data::UnsignedInt(v) => v.into_py(py),
            Data::SignedInt(v) => v.into_py(py),
            Data::Text(v) => v.into_py(py),
            Data::Array(v) => PyList::new(py, v).to_object(py),
            Data::Float(v) => v.into_py(py),
        }
    }
}

impl<'source> FromPyObject<'source> for Data {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        if ob.is_instance_of::<PyString>() {
            let str: String = ob.extract()?;
            return Ok(Data::Text(str))
        }
        if ob.is_instance_of::<PyLong>() || ob.is_instance_of::<PyInt>() {
            let i: i64 = ob.extract()?;
            return Ok(Data::SignedInt(i))
        }
        if ob.is_instance_of::<PyBool>() {
            let v: bool = ob.extract()?;
            return Ok(Data::Boolean(v))
        }
        if ob.is_instance_of::<PyList>() {
            let v: Vec<Data> = ob.extract()?;
            return Ok(Data::Array(v))
        }
        if ob.is_instance_of::<PyFloat>() {
            let v: f64 = ob.extract()?;
            return Ok(Data::Float(v))
        }
        Err(PyValueError::new_err("unrecognized type".to_string()))
    }
}