use crate::types::definition::block::code::github::Source;
use crate::utils;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Content {
    name: String,
    path: String,
    sha: String,
    size: i32,
    url: String,
    html_url: String,
    git_url: String,
    download_url: String,
    r#type: String,
    content: String,
    encoding: String,
    #[serde(rename = "_links")]
    links: Links,
}

#[derive(Serialize, Deserialize)]
struct Links {
    #[serde(rename = "self")]
    itself: String,
    git: String,
    html: String,
}

pub fn fetch_code(source: &Source) -> Result<String, String> {
    let path = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        source.owner, source.repository, source.path
    );
    let body = ureq::get(path.as_str())
        .set("Authorization", format!("Bearer {}", source.token).as_str())
        .set("X-GitHub-Api-Version", "2022-11-28")
        .set("Accept", " application/json")
        .call()
        .map_err(utils::to_string)?
        .into_string()
        .map_err(utils::to_string)?;
    let content = serde_json::from_str::<Content>(body.as_str()).map_err(utils::to_string)?;
    let source = ureq::get(content.download_url.as_str())
        .call()
        .map_err(utils::to_string)?
        .into_string()
        .map_err(utils::to_string)?;
    Ok(source)
}
