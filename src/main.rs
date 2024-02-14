mod api_client;

use api_client::{make_request, parse_headers, read_config};
use clap::Parser;
use colored::Colorize;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::{fs, path::Path};
use tokio::runtime::Runtime;

#[derive(Parser, Debug)]
#[clap(author = "kythonlk", version = "0.1", about = "Api clients run in terminal simplified same as httpie but you can give pre fixes headers and urls etc.", long_about = None , after_help = "EXAMPLE:\n     ex 1: fastreq -u 'http://example.com' -t g -b body.json -H headers.json -m       \n     ex 2: fastreq -c custom_config -t g -b body.json -H head.json -m")]
struct Args {
    /// URL to request (leave blank to use URL from config file and you can also give different config file)
    #[clap(short = 'u', long = "url")]
    url: Option<String>,

    /// Path to the configuration file
    #[clap(short = 'c', long = "config", default_value = "config.json")]
    config: String,

    /// HTTP request type (GET : g , POST : p , PUT : put, PATCH : patch)
    #[clap(short = 't', long = "type")]
    request_type: String,

    /// Path to the JSON body file
    #[clap(short = 'b', long = "body")]
    body_file: Option<String>,

    /// Path to the JSON headers file
    #[clap(short = 'H', long = "headers")]
    headers_file: Option<String>,

    /// Measure and print the time taken for the request
    #[clap(short = 'm', long = "time")]
    measure_time: bool,
}

fn main() {
    println!(
        "{} , {}",
        "FASTREQ CLI".bold().blue().italic(),
        "version 0.1".green().italic()
    );
    println!(
        "{}",
        "fast and simple http client with workspace feature"
            .italic()
            .purple()
    );

    let args = Args::parse();
    let runtime = Runtime::new().unwrap();

    let config = read_config(Path::new(&args.config)).expect("Failed to read config");
    let final_url = args.url.unwrap_or_else(|| config.url);

    let body = args
        .body_file
        .as_ref()
        .map(|path| fs::read_to_string(path).expect("Failed to read JSON body file"));

    let headers = if let Some(headers_file) = args.headers_file.as_ref() {
        let headers_json =
            fs::read_to_string(headers_file).expect("Failed to read JSON headers file");
        let headers_value: Value =
            serde_json::from_str(&headers_json).expect("Failed to parse headers JSON");
        parse_headers(&headers_value).expect("Failed to parse headers")
    } else {
        HeaderMap::new()
    };

    println!("Making a {} request to: {}", args.request_type, final_url);

    if args.measure_time {
        let start = std::time::Instant::now();
        let result = runtime.block_on(make_request(
            &final_url,
            &args.request_type,
            body.as_deref(),
            headers,
        ));
        let elapsed = start.elapsed();
        println!("Request completed in: {:?}", elapsed);
        handle_response(result);
    } else {
        let result = runtime.block_on(make_request(
            &final_url,
            &args.request_type,
            body.as_deref(),
            headers,
        ));
        handle_response(result);
    }
}

fn handle_response(result: Result<reqwest::Response, reqwest::Error>) {
    match result {
        Ok(response) => {
            if response.status().is_success() {
                let runtime = Runtime::new().unwrap();
                let body = runtime
                    .block_on(response.text())
                    .expect("Failed to read response body");

                match serde_json::from_str::<serde_json::Value>(&body) {
                    Ok(json) => println!("{}", serde_json::to_string_pretty(&json).unwrap()),
                    Err(_) => println!("Response: {}", body),
                }
            } else {
                println!("Request failed with status: {}", response.status());
            }
        }
        Err(e) => println!("Request error: {}", e),
    }
}
