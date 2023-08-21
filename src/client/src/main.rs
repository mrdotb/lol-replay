mod api;
mod recording;

use api::models::SpectatorEndpoint;
use api::utils::Region;
use recording::process;
use recording::storage::DiskStorage;

use clap::{Args, Parser};
use env_logger::{Builder, Env};

use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    // Using official league of legend spectator endpoint
    #[arg(long)]
    region_endpoint: Option<Region>,

    // Using custom league of legend spectator endpoint
    #[command(flatten)]
    custom_endpoint: Option<CustomEndpoint>,

    #[arg(long)]
    game_id: String,

    #[arg(long)]
    encryption_key: String,

    #[arg(long)]
    record_folder: PathBuf,
}

#[derive(Args, Debug)]
#[group(conflicts_with = "region_endpoint", multiple = true, required = false)]
struct CustomEndpoint {
    #[arg(long)]
    base_url: String,
    #[arg(long)]
    platform_id: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Cli::parse();

    env_logger::init();

    let endpoint = if let Some(region_endpoint) = args.region_endpoint {
        region_endpoint.to_endpoint()
    } else {
        let custom_endpoint = args.custom_endpoint.unwrap();
        SpectatorEndpoint::new(custom_endpoint.base_url, custom_endpoint.platform_id)
    };

    let storage = DiskStorage::new(args.record_folder.join(&endpoint.platform_id)).unwrap();

    process::new(
        endpoint,
        args.game_id,
        args.encryption_key,
        Box::new(storage),
    )
    .await?;

    Ok(())
}
