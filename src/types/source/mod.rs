use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;

#[typetag::serde(tag = "type")]
pub trait Source: Debug {
    fn code(&self) -> Result<String, String>;

    fn clone_source(&self) -> Box<dyn Source>;
}

impl Clone for Box<dyn Source> {
    fn clone(&self) -> Self {
        self.clone_source()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlainSource {
    pub code: String,
}

#[typetag::serde]
impl Source for PlainSource {
    fn code(&self) -> Result<String, String> {
        Ok(self.code.clone())
    }

    fn clone_source(&self) -> Box<dyn Source> {
        Box::new(PlainSource {
            code: self.code.clone(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GithubSource {
    pub owner: String,
    pub repository: String,
    pub path: String,
    pub token: String, //TODO - bad idea to place those thing in the db
}

#[typetag::serde]
impl Source for GithubSource {
    fn code(&self) -> Result<String, String> {
        Err("not implemented".to_string())
    }

    fn clone_source(&self) -> Box<dyn Source> {
        Box::new(self.clone())
    }
}
