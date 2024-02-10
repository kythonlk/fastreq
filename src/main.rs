use reqwest::{Client, Error, Response};
use std::io; // Removed the unused `Write` import
use tokio::runtime::Runtime;

async fn make_request(url: &str, method: &str) -> Result<Response, Error> {
    let client = Client::new();

    match method {
        "1" => client.get(url).send().await,  // GET
        "2" => client.post(url).send().await, // POST
        // Add more options as needed
        _ => unimplemented!("This method is not supported"),
    }
}

fn main() {
    let runtime = Runtime::new().unwrap(); // Removed the unnecessary `mut`

    println!("Enter the URL:");
    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");
    let url = url.trim(); // Remove newline character

    println!("Select the request type:");
    println!("1: GET");
    println!("2: POST");
    // Add more options as needed
    let mut method = String::new();
    io::stdin()
        .read_line(&mut method)
        .expect("Failed to read line");
    let method = method.trim(); // Remove newline character

    let method_name = match method {
        "1" => "GET",
        "2" => "POST",
        // Map more numbers to method names as needed
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
