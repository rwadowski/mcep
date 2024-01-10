use url::Url;

pub fn fetch(url: Url) -> Result<String, String> {
    ureq::post(url.as_str())
}

fn path(url: Url) -> String {}
