use std::collections::HashMap;
use crate::engine::block::Block;
use crate::engine::Data;
use crate::engine::router::Router;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationId(pub String);

//TODO - implement me
pub struct Application {
    blocks: Vec<Box<dyn Block>>,
    router: Router,
}

impl Application {

    fn run(&self, data: Data) -> Result<Data, String> {
        Err("not_implemented".to_string())
    }
}

struct Applications {
    apps: HashMap<String, Application>

}