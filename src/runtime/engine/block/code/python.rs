use crate::runtime::engine::Data;
use crate::types::definition::block::Dependency;
use crate::utils;
use log::debug;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyBool, PyFloat, PyInt, PyList, PyString};
use pyo3::IntoPyObjectExt;
use std::collections::HashMap;
use std::ffi::CString;

pub struct PythonBlock {
    pub source: String,
    pub dependencies: Vec<Dependency>,
}

impl PythonBlock {
    pub fn new(source: String, dependencies: Vec<Dependency>) -> PythonBlock {
        PythonBlock {
            source,
            dependencies,
        }
    }
    pub fn run(&self, input: HashMap<String, Data>) -> Result<HashMap<String, Data>, String> {
        debug!("running python code block \n{}", self.source);
        Python::attach(|py| {
            for dependency in self.dependencies.iter() {
                PyModule::import(py, dependency.name.as_str()).map_err(utils::to_string)?;
            }
            let code = CString::new(self.source.as_str()).map_err(utils::to_string)?;
            let module = PyModule::from_code(py, code.as_c_str(), c"logic.py", c"logic")
                .map_err(utils::to_string)?;
            let function: Py<PyAny> = module.getattr("logic").map_err(utils::to_string)?.into();
            let args = (input.into_py_dict(py).map_err(utils::to_string)?,);
            let result = function.call(py, args, None);
            match result {
                Ok(object) => {
                    let map: HashMap<String, Data> =
                        object.extract(py).map_err(utils::to_string)?;
                    Ok(map)
                }
                Err(e) => Err(e.to_string()),
            }
        })
    }
}

impl<'py> IntoPyObject<'py> for Data {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            Data::Boolean(v) => v.into_bound_py_any(py),
            Data::UnsignedInt(v) => v.into_bound_py_any(py),
            Data::SignedInt(v) => v.into_bound_py_any(py),
            Data::Text(v) => v.into_bound_py_any(py),
            Data::Float(v) => v.into_bound_py_any(py),
            Data::Array(v) => PyList::new(py, v)?.into_bound_py_any(py),
        }
    }
}

impl<'source, 'py> FromPyObject<'source, 'py> for Data {
    type Error = PyErr;
    fn extract(ob: Borrowed<'source, 'py, PyAny>) -> Result<Self, Self::Error> {
        if ob.is_instance_of::<PyString>() {
            let str: String = ob.extract()?;
            return Ok(Data::Text(str));
        }
        if ob.is_instance_of::<PyInt>() {
            let value: i64 = ob.extract()?;
            return Ok(Data::SignedInt(value));
        }
        if ob.is_instance_of::<PyBool>() {
            let v: bool = ob.extract()?;
            return Ok(Data::Boolean(v));
        }
        if ob.is_instance_of::<PyList>() {
            let v: Vec<Data> = ob.extract()?;
            return Ok(Data::Array(v));
        }
        if ob.is_instance_of::<PyFloat>() {
            let v: f64 = ob.extract()?;
            return Ok(Data::Float(v));
        }
        Err(PyValueError::new_err("unrecognized type".to_string()))
    }
}
