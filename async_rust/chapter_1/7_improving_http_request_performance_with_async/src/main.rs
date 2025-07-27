use std::time::{Duration, Instant};

use reqwest::Error;
use serde::Deserialize;
use serde_json;
use tokio::time::sleep;

#[derive(Deserialize, Debug)]
struct Response {
    url: String,
    args: serde_json::Value,
}

async fn fetch_data(seconds: u64) -> Result<Response, Error> {
    let url = format!("https://httpbin.org/delay/{}", seconds);
    let response = reqwest::get(url).await?;
    let data = response.json().await?;
    Ok(data)
}

async fn calculate_last_login() {
    sleep(Duration::from_secs(1)).await;
    println!("Logged in 2 days ago");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let start_time = Instant::now();
    let data = fetch_data(5);
    let time_since = calculate_last_login();
    let (data, _) = tokio::join!(data, time_since);
    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);
    if let Ok(data) = data {
        println!("Response URL: {}", data.url);
        println!("Arguments: {}", data.args);
    } else {
        eprintln!("Failed to fetch data");
    }
    Ok(())
}
