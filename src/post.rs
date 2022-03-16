use custom_error::custom_error;

custom_error! {pub Error
    ReqwestError{source: reqwest::Error} = "{source}",
    ParseError{source: serde_json::Error} = "{source}",
    AnkiError{err: String} = "{err}"
}

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;

pub fn post(client: Client, headers: HeaderMap, body: &'static str) -> Result<Value, Error> {
    let result = post_raw(client, headers, body)?;
    let json: Value = serde_json::from_str(&result)?;
    match &json["error"] {
        Value::Null => Ok(json),
        err => Err(Error::AnkiError {
            err: err.to_string(),
        }),
    }
}

pub fn post_raw(
    client: Client,
    headers: HeaderMap,
    body: &'static str,
) -> Result<String, reqwest::Error> {
    Ok(client
        .post("http://127.0.0.1:8765")
        .body(body)
        .headers(headers)
        .send()?
        .text()?)
}

pub fn get_headers() -> HeaderMap {
    let mut header = HeaderMap::new();
    header.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    header
}
