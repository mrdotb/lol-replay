mod api;
mod recording;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();

    let endpoint = api::models::SpectatorEndpoint {
        base_url: "http://replays.leagueofgraphs.com:80".to_string(),
        platform_id: "EUW1-2964515075".to_string(),
    };
    // let endpoint = api::utils::Region::KR.to_endpoint();
    let storage =
        recording::storage::DiskStorage::new("./replays/EUW1/6557044068/".into()).unwrap();

    recording::process::new(
        endpoint,
        "6557044068".to_string(),
        "6zddQdKEBzH5dDqd/8igd8k4JFaPBjKy".to_string(),
        Box::new(storage),
    )
    .await?;

    // recording::process::new(
    //     endpoint,
    //     "6660621981".to_string(),
    //     "qsQv0PgN7VQPjgzNhE+DYgPAFOYn1fwl".to_string()
    //     ).await?;

    Ok(())
}
