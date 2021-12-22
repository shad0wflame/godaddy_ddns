mod go_daddy_ddns;
mod ip_handler;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let domain = env::var("DOMAIN").unwrap();
    let key = env::var("KEY").unwrap();
    let secret = env::var("SECRET").unwrap();

    go_daddy_ddns::exec(&domain, &key, &secret).await
}
