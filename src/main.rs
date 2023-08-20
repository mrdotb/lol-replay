mod api;
mod recording;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();

    // let endpoint = api::models::SpectatorEndpoint {
    //     base_url: "http://replays.leagueofgraphs.com:80".to_string(),
    //     platform_id: "KR-2964515075".to_string(),
    // };
    // let endpoint = api::utils::Region::KR.to_endpoint();
    // let storage =
    //     recording::storage::DiskStorage::new("./replays/KR/".into()).unwrap();

    // recording::process::new(
    //     endpoint,
    //     "6663346724".to_string(),
    //     "4jZMmo+rCNgS72PFof62lgQNWNXfW0jb".to_string(),
    //     Box::new(storage),
    // )
    // .await?;

    // recording::process::new(
    //     endpoint,
    //     "6663292320".to_string(),
    //     "OUA0tFOAfPMTx6IPMQlqyI4seOHZrppz".to_string(),
    //     Box::new(storage),
    //     ).await?;

    Ok(())
}
