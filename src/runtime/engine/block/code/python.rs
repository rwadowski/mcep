use crate::runtime::engine::block::code::PythonCodeBlock;
use crate::runtime::engine::Data;
use crate::utils;
use http::uri::InvalidUri;
use http::Uri;
use log::debug;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::{PyBool, PyFloat, PyInt, PyList, PyLong, PyString};
use std::collections::HashMap;
use url::Url;

pub struct PythonBlock {
    code: String,
}

impl PythonBlock {
    pub fn new(code: String) -> Result<PythonBlock, String> {
        let content = load(code)?;
        Ok(PythonBlock { code: content })
    }

    pub fn run(&self, input: HashMap<String, Data>) -> Result<HashMap<String, Data>, String> {
        debug!("running python code block {}", self.code);
        let code = self.load()?; //TODO - move it to creation of the PythonBlock instance - it is an expensive operation
        Python::with_gil(|py| {
            let function: Py<PyAny> = PyModule::from_code(py, code.as_str(), "", "")
                .map_err(|e| e.to_string())?
                .getattr("logic")
                .map_err(|e| e.to_string())?
                .into();
            let args = (input.into_py_dict(py),);
            let result = function.call(py, args, None);
            match result {
                Ok(object) => {
                    let map: HashMap<String, Data> =
                        object.extract(py).map_err(|e| e.to_string())?;
                    Ok(map)
                }
                Err(e) => Err(e.to_string()),
            }
        })
    }
}

fn load(code: String) -> Result<String, String> {
    let url_opt = Url::parse(code.as_str());
    match url_opt {
        Ok(url) => fetch_code(url),
        Err(_) => Ok(code.clone()),
    }
}

fn fetch_code(url: Url) -> Result<String, String> {
    let response = ureq::get(url.as_str()).call().map_err(utils::to_string)?;
    response.into_string().map_err(utils::to_string)
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
            return Ok(Data::Text(str));
        }
        if ob.is_instance_of::<PyLong>() || ob.is_instance_of::<PyInt>() {
            let i: i64 = ob.extract()?;
            return Ok(Data::SignedInt(i));
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
