use crate::types::definition::block::code::github::{Github, Source};
use crate::types::definition::block::{Dependency, Input, Output};
use crate::types::definition::{DataType, Definition};
use crate::utils;

pub fn new_store_block_definition(token: String) -> Result<Definition, String> {
    let github = Github {
        inputs: vec![
            Input::new("x", DataType::Text),
            Input::new("y", DataType::Text),
        ],
        outputs: vec![Output::new("z", DataType::Text)],
        source: Source {
            owner: "rwadowski".to_string(),
            repository: "mcep-scripts".to_string(),
            token,
            path: "store.py".to_string(),
        },
        dependencies: vec![Dependency {
            name: "psycopg2".to_string(),
        }],
    };
    let body = utils::json::serialize_to_value_with_type_tag(&github, "Github")
        .map_err(utils::to_string)?;
    Ok(Definition {
        id: 0,
        name: "psycopg2".to_string(),
        version: "tools-v1.0".to_string(),
        body,
        description: None,
        help: None,
    })
}
pub fn new_sum_block_definition(token: String) -> Result<Definition, String> {
    let github = Github {
        inputs: vec![
            Input::new("x", DataType::Text),
            Input::new("y", DataType::Text),
        ],
        outputs: vec![Output::new("z", DataType::Text)],
        source: Source {
            owner: "rwadowski".to_string(),
            repository: "mcep-scripts".to_string(),
            token,
            path: "sum.py".to_string(),
        },
        dependencies: Vec::new(),
    };
    let body = utils::json::serialize_to_value_with_type_tag(&github, "Github")
        .map_err(utils::to_string)?;
    Ok(Definition {
        id: 0,
        name: "sum".to_string(),
        version: "tools-v1.0".to_string(),
        body,
        description: None,
        help: None,
    })
}
