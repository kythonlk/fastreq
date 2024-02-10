use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Error, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub headers: Value,
}

pub async fn make_request(url: &str, headers: HeaderMap) -> Result<Response, Error> {
    let client = Client::new();
    client.get(url).headers(headers).send().await
}

pub fn read_config(file_path: &Path) -> std::io::Result<Config> {
    let config_str = fs::read_to_string(file_path)?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}

pub fn parse_headers(headers_json: &Value) -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    if let Value::Object(map) = headers_json {
        for (key, value) in map {
            if let Value::String(value_str) = value {
                headers.insert(
                    HeaderName::from_bytes(key.as_bytes())?,
                    HeaderValue::from_str(&value_str)?,
                );
            }
        }
    }
    Ok(headers)
}
