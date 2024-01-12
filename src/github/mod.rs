use url::Url;

pub struct GithubCode {
    pub url: Url,
    pub owner: String,
    pub repository: String,
    pub token: String,
}

impl GithubCode {
    pub fn new(path: String) -> GithubCode {
        GithubCode {
            url: Url::parse("").unwrap(),
            owner: "rwadowski".to_string(),
            repository: "mcep-scripts".to_string(),
            token: "".to_string(),
            // path: "",
        }
    }

    fn url(&self) -> Url {}
}

pub fn fetch(code: GithubCode) -> Result<String, String> {
    ureq::post(url.as_str())
}

fn path(url: Url) -> String {}
