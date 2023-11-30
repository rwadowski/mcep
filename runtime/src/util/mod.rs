use serde::de::DeserializeOwned;
use std::fs;

pub fn load<T: DeserializeOwned>(file_name: String) -> Result<T, String> {
    let contents = fs::read_to_string(file_name).map_err(|e| e.to_string())?;
    let data: T = toml::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(data)
}
