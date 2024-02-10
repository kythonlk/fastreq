use reqwest::{Client, Error, Response};
use std::io;
use tokio::runtime::Runtime;

async fn make_request(url: &str, method: &str) -> Result<Response, Error> {
    let client = Client::new();

    match method {
        "1" => client.get(url).send().await,  // GET
        "2" => client.post(url).send().await, // POST
        _ => unimplemented!("This method is not supported"),
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();
    println!("Enter the URL:");
    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");
    let url = url.trim();

    println!("Select the request type:");
    println!("1: GET");
    println!("2: POST");
    let mut method = String::new();
    io::stdin()
        .read_line(&mut method)
        .expect("Failed to read line");
    let method = method.trim();

    let method_name = match method {
        "1" => "GET",
        "2" => "POST",
        _ => {
            println!("Invalid option");
            return;
        }
    };

    println!("Making a {} request to: {}", method_name, url);

    match runtime.block_on(make_request(url, method)) {
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
