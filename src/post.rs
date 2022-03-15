use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

pub fn post(
    client: Client,
    headers: HeaderMap,
    body: &'static str,
) -> Result<String, reqwest::Error> {
    Ok(client
        .post("http://127.0.0.1:8765")
        .body(format!("{{\"version\": 6, {} }}", body))
        .headers(headers)
        .send()?
        .text()?)
}

pub fn get_headers() -> HeaderMap {
    let mut header = HeaderMap::new();
    header.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    header
}
