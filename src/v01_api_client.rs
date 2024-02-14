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

pub async fn make_request(
    url: &str,
    method: &str,
    body: Option<&str>,
    headers: HeaderMap,
) -> Result<Response, Error> {
    let client = Client::new();

    let request_builder = match method {
        "1" => client.get(url),
        "2" => {
            let json_body: Value =
                serde_json::from_str(body.unwrap_or("{}")).expect("Invalid JSON body");
            client.post(url).json(&json_body)
        }
        "3" => client.put(url).body(body.unwrap_or_default().to_string()),
        "4" => client.patch(url).body(body.unwrap_or_default().to_string()),
        _ => unimplemented!("This method is not supported"),
    };

    request_builder.headers(headers).send().await
}

pub fn read_json_file(file_path: &Path) -> std::io::Result<String> {
    fs::read_to_string(file_path)
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

pub fn read_config(file_path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(file_path)?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}
