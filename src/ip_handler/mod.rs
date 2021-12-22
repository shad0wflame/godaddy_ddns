use serde::Deserialize;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

const IP_FILE_NAME: &'static str = "ddns_ip";
const WEBSITE_URL: &'static str = "https://httpbin.org/ip";

#[derive(Deserialize)]
struct IP {
    origin: String,
}

/// Returns an Option holding the current IP address if the value has changed, otherwise returns None.
pub async fn get_ip_to_publish() -> Option<String> {
    let previous_ip = match check_previous_ip() {
        Some(x) => x,
        None => String::new(),
    };

    let current_ip = check_current_ip()
        .await
        .expect("Error getting the current IP.");

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
fn record_current_ip(current_ip: &str) -> () {
    let mut file = File::create(IP_FILE_NAME).expect("Error creating file.");
    file.write_all(current_ip.as_ref())
        .expect("Error writing file.");
}

/// Reads the current IP value from the FILE_NAME and returns it.
fn check_previous_ip() -> Option<String> {
    let path = Path::new(IP_FILE_NAME);

    if !path.exists() {
        return None;
    }

    let contents = read_to_string(path).expect("Error reading file.");

    Some(contents)
}
