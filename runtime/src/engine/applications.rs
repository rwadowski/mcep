use std::collections::HashMap;
use crate::engine::block::Block;
use crate::engine::Data;
use crate::engine::router::Router;

pub(crate) struct ApplicationId(pub String);

pub(crate) struct Application {
    blocks: Vec<Box<dyn Block>>,
    router: Router,
    // input: Receiver<Data>,
    // sender: Sender<Data>,
}

impl Application {

    fn run(&self, data: Data) -> Result<Data, String> {
        Err("not_implemented".to_string())
    }
}

struct Applications {
    apps: HashMap<String, Application>

}