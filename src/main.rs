mod go_daddy_ddns;
mod ip_handler;

use go_daddy_ddns::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: [GoDaddy] - Get the domain from ENV variable to replace "a".
    // TODO: [GoDaddy] - Get the key from ENV variable to replace "b".
    // TODO: [GoDaddy] - Get the secret from ENV variable to replace "c".

    go_daddy_ddns::exec("a", "b", "c").await
}
