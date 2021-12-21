use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = get_current_ip().await?;

    println!("{:#?}", ip);

    Ok(())
}

async fn get_current_ip() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    Ok(resp.get("origin").unwrap().as_str().parse().unwrap())
}
