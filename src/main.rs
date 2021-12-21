use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::{Serialize, Deserialize};

const FILE_NAME: &'static str = "ddns_ip";

#[derive(Deserialize)]
struct IP {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_ip = get_current_ip().await?;

    let previous_ip = match check_previous_ip() {
        Some(x) => x,
        None => {
            let mut file = File::create(FILE_NAME)?;
            file.write_all(current_ip.as_ref())?;

            String::new()
        }
    };

    println!("Current IP: {}, Previous IP: {}", current_ip, previous_ip);

    if current_ip.ne(&previous_ip) {
        println!("Not equal!!!");
        // TODO: GoDaddy connection.
    }

    Ok(())
}

async fn get_current_ip() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<IP>()
        .await?;

    Ok(resp.origin)
}

fn check_previous_ip() -> Option<String> {
    if !Path::new(FILE_NAME).exists() {
        return None;
    }

    let mut file = File::open(FILE_NAME).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    return Some(contents);
}
