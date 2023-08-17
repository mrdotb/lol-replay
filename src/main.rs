mod api;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();

    // let version =
    //     api::endpoints::fetch_game_meta_data(&api::utils::Region::KR, "6654667050").await?;
    // println!("{}", version);

    Ok(())
}
