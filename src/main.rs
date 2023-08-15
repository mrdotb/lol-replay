// mod api;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();
    // let url = format!("{}{}", BASE_URL, "/observer-mode/rest/consumer/version");
    // let response = reqwest::get(url).await?;
    // println!("{:#?}", response);
    // let test = api::utils::base_url(&api::utils::Region::KR);

    // let _version =
    //     api::endpoints::fetch_game_meta_data(&api::utils::Region::KR, "6654667050").await?;
    // println!("{}", version);

    Ok(())
}
