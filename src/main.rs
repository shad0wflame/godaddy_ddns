mod ip_handler;

use ip_handler::ip_has_changed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    if ip_has_changed().await {
        println!("Ip has changed!");
    }

    Ok(())
}
