use apex_deployer::{authorize, update, UpdateConfig};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let config = UpdateConfig::configure(&args[1], &args[2], HashMap::from([(&args[3], &args[4])]));
    let token: String = authorize().await?.access_token;
    update(&token, config).await?;
    Ok(())
}
