use std::env;

use log::LevelFilter;
use simple_logger::SimpleLogger;

mod go_daddy_ddns;
mod ip_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_colors(true)
        .with_utc_timestamps()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let domain = env::var("DOMAIN").expect("You need to set DOMAIN env variable first.");
    let key = env::var("KEY").expect("You need to set KEY env variable first.");
    let secret = env::var("SECRET").expect("You need to set SECRET env variable first.");

    go_daddy_ddns::exec(&domain, &key, &secret).await
}
