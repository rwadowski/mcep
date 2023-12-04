use crate::deployment::connection::junction::BlockJunction;
use serde::{Deserialize, Serialize};

pub mod junction;
mod junction_test;
mod mod_test;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct BlockConnection {
    pub from: BlockJunction,
    pub to: BlockJunction,
}

impl BlockConnection {}
