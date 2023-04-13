use dotenv::dotenv;
use sf_updates::{authorize, get_ids, update, UpdateConfig};
use std::collections::HashMap;
mod models {
    pub mod arg_model;
    pub mod model;
}
use crate::models::arg_model::{Args, Get};

use clap::Parser;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::try_parse();

    let token: String = authorize().await?.access_token;
    match args {
        Ok(a) => {
            let fields_copied = a.fields;

            let fields_copied = fields_copied
                .chunks_exact(2)
                .map(|chunk| (&chunk[0], &chunk[1]))
                .collect::<HashMap<_, _>>();
            let config = UpdateConfig::configure(&a.sobj, &a.id, fields_copied);
            update(&token, config).await?;
        }
        Err(_) => {
            let get = Get::parse();
            let _ids: String = get_ids(&token, &get.sobj, &get.name).await?;
        }
    }
    Ok(())
}
