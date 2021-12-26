use std::env;
use std::error::Error;

use log::LevelFilter;
use simple_logger::SimpleLogger;

mod file_handler;
mod records_handler;
mod ip_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_colors(true)
        .with_utc_timestamps()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    file_handler::application_folder_setup().expect("Error setting up application folder.");

    let domain = env::var("DOMAIN").expect("You need to set DOMAIN env variable first.");
    let key = env::var("KEY").expect("You need to set KEY env variable first.");
    let secret = env::var("SECRET").expect("You need to set SECRET env variable first.");

    records_handler::update(&domain, &key, &secret).await
}
