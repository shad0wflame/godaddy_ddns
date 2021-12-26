use log::debug;

use crate::file_handler::{get_ip_file, set_ip_file};
use crate::ip_handler::ip::IP;

mod ip;

const WEBSITE_URL: &'static str = "https://httpbin.org/ip";

/// Returns an Option holding the current IP address if the value has changed, otherwise returns None.
pub async fn get_ip_to_publish() -> Option<String> {
    let previous_ip = match check_previous_ip() {
        Some(x) => x,
        None => String::new(),
    };

    let current_ip = check_current_ip()
        .await
        .expect("Error getting the current IP.");

    debug!("Current IP: {}, Previous IP: {}", current_ip, previous_ip);

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

    set_ip_file((&resp.origin).as_ref());

    Ok(resp.origin)
}

/// Reads the current IP value from the FILE_NAME and returns it.
fn check_previous_ip() -> Option<String> {
    get_ip_file()
}
