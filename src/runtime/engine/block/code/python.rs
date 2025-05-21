use crate::runtime::engine::Data;
use crate::types::definition::block::Dependency;
use log::debug;
use rustpython::vm;
use rustpython::vm::builtins::{PyDict, PyStr};
use rustpython::vm::object::Py;
use rustpython::vm::{AsObject, Interpreter, PyObjectRef, TryFromObject, VirtualMachine};
use std::collections::HashMap;
use std::ops::Deref;

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
        let interpreter = Interpreter::with_init(Default::default(), |vm| {
            vm.add_native_modules(vm::stdlib::get_module_inits());
        });
        let result = interpreter.enter(|vm| {
            let scope = vm.new_scope_with_builtins();
            vm.run_code_string(scope.clone(), self.source.as_str(), "".to_string())
                .map_err(|e| "python code execution error".to_string())
                .expect("fix me");
            let logic_fn: PyObjectRef = scope.globals.get_item("logic", vm).expect("fix me");
            let dict = to_dict(vm, &input).expect("failed to get a dictionary");
            let args = vec![dict];
            let result = logic_fn.call(args, vm).expect("fix me");
            let data_map: HashMap<String, Data> = from_dict(vm, result)
                .map_err(|e| "python code execution error".to_string())
                .expect("fix me");
            data_map
        });
        Ok(result)
        // Python::with_gil(|py| {
        //     for dependency in self.dependencies.iter() {
        //         PyModule::import_bound(py, dependency.name.as_str()).map_err(utils::to_string)?;
        //     }
        //     let module = PyModule::from_code_bound(py, self.source.as_str(), "logic.py", "logic")
        //         .map_err(utils::to_string)?;
        //     let function: Py<PyAny> = module.getattr("logic").map_err(utils::to_string)?.into();
        //     let args = (input.into_py_dict_bound(py),);
        //     let result = function.call_bound(py, args, None);
        //     match result {
        //         Ok(object) => {
        //             let map: HashMap<String, Data> =
        //                 object.extract(py).map_err(utils::to_string)?;
        //             Ok(map)
        //         }
        //         Err(e) => Err(e.to_string()),
        //     }
        // })
    }
}

fn to_dict(vm: &VirtualMachine, data: &HashMap<String, Data>) -> Result<PyObjectRef, String> {
    let dict = vm.ctx.new_dict();
    let scope = vm.new_scope_with_builtins();
    for (key, value) in data {
        let k = vm.ctx.new_str(key.to_string());
        let dict_key: &Py<PyStr> = k.deref();
        let py_value: PyObjectRef = to_value(vm, value);
        dict.set_item::<Py<PyStr>>(dict_key, py_value, vm)
            .map_err(|e| "failed to convert into a dictionary")?;
    }
    Ok(PyObjectRef::from(dict))
}

fn to_value(vm: &VirtualMachine, data: &Data) -> PyObjectRef {
    match data {
        Data::Boolean(v) => PyObjectRef::from(vm.ctx.new_bool(v.clone())),
        Data::UnsignedInt(v) => PyObjectRef::from(vm.ctx.new_int(v.clone())),
        Data::SignedInt(v) => PyObjectRef::from(vm.ctx.new_int(v.clone())),
        Data::Float(v) => PyObjectRef::from(vm.ctx.new_float(v.clone())),
        Data::Text(s) => PyObjectRef::from(vm.ctx.new_str(s.clone())),
        Data::Array(a) => {
            let mut list = Vec::new();
            for d in a.iter() {
                list.push(to_value(vm, d));
            }
            PyObjectRef::from(vm.ctx.new_list(list))
        }
    }
}
fn from_dict(vm: &VirtualMachine, obj: PyObjectRef) -> Result<HashMap<String, Data>, String> {
    let py_dict = obj
        .downcast::<PyDict>()
        .map_err(|_| "Expected a Python dictionary".to_string())?;

    let mut map = HashMap::new();
    for (key, value) in py_dict.into_iter() {
        let key_str = key
            .downcast::<PyStr>()
            .map_err(|_| "Expected a string as key in dictionary".to_string())?;
        let key_value = key_str.to_string();
        let data_value =
            to_data(vm, &value).map_err(|_| "Failed to convert value to Data".to_string())?;
        map.insert(key_value, data_value);
    }

    Ok(map)
}

fn to_data(vm: &VirtualMachine, obj: &PyObjectRef) -> Result<Data, String> {
    if let Ok(true) = obj.clone().is_instance(vm.ctx.types.bool_type.as_ref(), vm) {
        let v = bool::try_from_object(vm, obj.clone())
            .map_err(|e| e.as_object().str(vm).unwrap().to_string())?;
        return Ok(Data::Boolean(v));
    }
    if let Ok(true) = obj
        .clone()
        .is_instance(vm.ctx.types.float_type.as_ref(), vm)
    {
        let v = i64::try_from_object(vm, obj.clone())
            .map_err(|e| e.as_object().str(vm).unwrap().to_string())?;
        return Ok(Data::SignedInt(v));
    }
    if let Ok(true) = obj.clone().is_instance(vm.ctx.types.int_type.as_ref(), vm) {
        let v: f64 = f64::try_from_object(vm, obj.clone())
            .map_err(|e| e.as_object().str(vm).unwrap().to_string())?;
        return Ok(Data::Float(v));
    }
    if let Ok(true) = obj.clone().is_instance(vm.ctx.types.str_type.as_ref(), vm) {
        let v = String::try_from_object(vm, obj.clone())
            .map_err(|e| e.as_object().str(vm).unwrap().to_string())?;
        return Ok(Data::Text(v));
    }
    if let Ok(true) = obj.clone().is_instance(vm.ctx.types.list_type.as_ref(), vm) {
        let seq = obj.to_sequence();
        let length = seq
            .length(vm)
            .map_err(|e| e.as_object().str(vm).unwrap().to_string())? as isize;
        let mut list = Vec::new();
        for i in 0..length {
            let item = to_data(
                vm,
                &seq.get_item(i, vm)
                    .map_err(|e| e.as_object().str(vm).unwrap().to_string())?,
            )?;
            list.push(item);
        }
        return Ok(Data::Array(list));
    }
    // if let Ok(v) = obj.clone().try_into_value::<String>(vm) {

    // return Ok(Data::Text(v));
    // }
    //
    // if let Ok(true) = obj.is_instance(vm.ctx.types.list_type.as_ref(), vm) {
    //     // let o = obj.payload::<PyList>().unwrap();
    //     let o = obj.
    //     let elements = o.payload::<PyList>();
    // }
    // if let Ok(py_list) = obj.clone().downcast::<PyList>() {
    //     let mut vec = Vec::new();
    //     let x = py_list
    //     for item in py_list {
    //         vec.push(Data::try_from(item)?);
    //     }
    //     return Ok(Data::Array(vec));
    // }
    Err("type not recognized".to_string())
}
