use crate::ip_handler::get_ip_to_publish;

/// Updates the DNS records if the IP has changed and returns the result of the execution.
///
/// # Arguments
///
/// * `domain` - A &str holding the domain to update.
///
/// * `key` - A &str holding the GoDaddy developer key.
///
/// * `secret` - A &str holding the GoDaddy developer secret.
pub async fn exec(domain: &str, key: &str, secret: &str) -> Result<(), Box<dyn std::error::Error>> {
    let new_ip = get_ip_to_publish().await;

    /// There's no need to do anything here. So we stop the execution.
    if Option::is_none(&new_ip) {
        return Ok(());
    }

    // TODO: Create a struct representing the structure of
    //  a JSON file including the DNS Records we want to update.

    // TODO: Create that JSON file.

    // TODO: Read that JSON file and deserialize it with serde.

    // TODO: Update the DNS Records.

    Ok(())
}
