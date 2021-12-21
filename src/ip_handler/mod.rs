use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const FILE_NAME: &'static str = "ddns_ip";
const WEBSITE_URL: &'static str = "https://httpbin.org/ip";

#[derive(Deserialize)]
struct IP {
    origin: String,
}

/// Returns an Option holding the current IP address if the value has changed, otherwise returns None.
pub async fn get_ip_to_publish() -> Option<String> {
    let current_ip = check_current_ip()
        .await
        .expect("Error getting the current IP.");

    let previous_ip = match check_previous_ip() {
        Some(x) => x,
        None => String::new(),
    };

    println!("Current IP: {}, Previous IP: {}", current_ip, previous_ip);

    if current_ip.eq(&previous_ip) {
        return None;
    }

    Some(current_ip)
}

/// Checks the current WAN IP.
///
/// Connects to WEBSITE_URL and retrieves the current WAN IP value.
async fn check_current_ip() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(WEBSITE_URL).await?.json::<IP>().await?;

    record_current_ip(&resp.origin);

    Ok(resp.origin)
}

/// Stores the current IP value in the FILE_NAME.
///
/// # Arguments
///
/// * `current_ip` - A &str holding the current IP value.
fn record_current_ip(current_ip: &str) {
    let mut file = File::create(FILE_NAME).expect("Error creating file.");
    file.write_all(current_ip.as_ref())
        .expect("Error writing file.");
}

/// Reads the current IP value from the FILE_NAME and returns it.
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
