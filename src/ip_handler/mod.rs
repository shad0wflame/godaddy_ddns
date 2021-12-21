use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};

const FILE_NAME: &'static str = "ddns_ip";
const WEBSITE_URL: &'static str = "https://httpbin.org/ip";

#[derive(Deserialize)]
struct IP {
    origin: String,
}

pub async fn ip_has_changed() -> bool {
    let current_ip = get_current_ip().await.expect("Error getting the current IP.");

    let previous_ip = match check_previous_ip() {
        Some(x) => x,
        None => String::new()
    };

    println!("Current IP: {}, Previous IP: {}", current_ip, previous_ip);

    current_ip.ne(&previous_ip)
}

async fn get_current_ip() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(WEBSITE_URL)
        .await?
        .json::<IP>()
        .await?;

    record_current_ip(&resp.origin);

    Ok(resp.origin)
}

fn record_current_ip(current_ip: &str)  {
    let mut file = File::create(FILE_NAME).expect("Error creating file.");
    file.write_all(current_ip.as_ref()).expect("Error writing file.");
}

fn check_previous_ip() -> Option<String> {
    if !Path::new(FILE_NAME).exists() {
        return None;
    }

    let mut file = File::open(FILE_NAME).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file.");

    return Some(contents);
}
