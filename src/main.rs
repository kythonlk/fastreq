mod api_client;

use api_client::{make_request, parse_headers, read_config, read_json_file};
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::{io, path::Path};
use tokio::runtime::Runtime;

fn main() {
    let runtime = Runtime::new().unwrap();

    println!("Enter the URL (or leave blank to use URL from config.json):");
    let mut url_input = String::new();
    io::stdin()
        .read_line(&mut url_input)
        .expect("Failed to read line");
    let url_input = url_input.trim();

    println!("Select the request type:");
    println!("1: GET");
    println!("2: POST");
    println!("3: PUT");
    println!("4: PATCH");
    let mut method = String::new();
    io::stdin()
        .read_line(&mut method)
        .expect("Failed to read line");
    let method = method.trim();

    let body: Option<String> = if ["2", "3", "4"].contains(&method) {
        println!("Enter the path to the JSON body file (e.g., body.json):");
        let mut body_file_path = String::new();
        io::stdin()
            .read_line(&mut body_file_path)
            .expect("Failed to read line");
        let body_file_path = Path::new(body_file_path.trim());
        match read_json_file(body_file_path) {
            Ok(content) => Some(content),
            Err(e) => {
                eprintln!("Failed to read JSON body file: {}", e);
                return;
            }
        }
    } else {
        None
    };

    let mut headers = HeaderMap::new();
    println!("Do you want to add headers from a file? (y/n):");
    let mut decision = String::new();
    io::stdin()
        .read_line(&mut decision)
        .expect("Failed to read line");
    if decision.trim().eq_ignore_ascii_case("y") {
        println!("Enter the path to the JSON headers file (e.g., headers.json):");
        let mut headers_file_path = String::new();
        io::stdin()
            .read_line(&mut headers_file_path)
            .expect("Failed to read line");
        let headers_file_path = Path::new(headers_file_path.trim());
        let headers_json = match read_json_file(headers_file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read JSON headers file: {}", e);
                return;
            }
        };

        let headers_value: Value = match serde_json::from_str(&headers_json) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Failed to parse headers JSON: {}", e);
                return;
            }
        };

        headers = match parse_headers(&headers_value) {
            Ok(parsed_headers) => parsed_headers,
            Err(e) => {
                eprintln!("Failed to parse headers: {}", e);
                return;
            }
        };
    }

    let final_url = if url_input.is_empty() {
        let config = match read_config(Path::new("config.json")) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("Failed to read config: {}", e);
                return;
            }
        };
        config.url
    } else {
        url_input.to_string()
    };

    println!("Making a {} request to: {}", method, final_url);

    match runtime.block_on(make_request(&final_url, method, body.as_deref(), headers)) {
        Ok(response) => {
            if response.status().is_success() {
                let body = runtime
                    .block_on(response.text())
                    .expect("Failed to read response body");
                println!("Response: {}", body);
            } else {
                println!("Request failed with status: {}", response.status());
            }
        }
        Err(e) => println!("Request error: {}", e),
    }
}
