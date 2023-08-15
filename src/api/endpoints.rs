use super::utils::{base_url, Region};
use log::debug;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameMetaData {
    pub game_key: GameKey,
    pub game_server_address: String,
    pub port: u32,
    pub encryption_key: String,
    pub chunk_time_interval: u32,
    pub start_time: String,
    pub game_ended: bool,
    pub last_chunk_id: u32,
    pub last_key_frame_id: u32,
    pub end_startup_chunk_id: u32,
    pub delay_time: u32,
    pub pending_available_chunk_info: Vec<PendingAvailableChunkInfo>,
    pub pending_available_key_frame_info: Vec<PendingAvailableKeyFrameInfo>,
    pub key_frame_time_interval: u64,
    pub decoded_encryption_key: String,
    pub start_game_chunk_id: u32,
    pub game_length: u32,
    pub client_added_lag: u32,
    pub client_back_fetching_enabled: bool,
    pub client_back_fetching_freq: u32,
    pub interest_score: u32,
    pub featured_game: bool,
    pub create_time: String,
    pub end_game_chunk_id: i32,
    pub end_game_key_frame_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameKey {
    pub game_id: u64,
    pub platform_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingAvailableChunkInfo {
    pub chunk_id: u32,
    pub duration: u32,
    pub received_time: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingAvailableKeyFrameInfo {
    pub key_frame_id: u32,
    pub received_time: String,
    pub next_chunk_id: u32,
}

impl fmt::Display for GameMetaData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game Key: {:?}\n", self.game_key)?;
        write!(f, "Server Address: {}\n", self.game_server_address)?;
        write!(f, "Port: {}\n", self.port)?;
        write!(f, "Encryption Key: {}\n", self.encryption_key)?;
        write!(f, "Chunk Time Interval: {}\n", self.chunk_time_interval)?;
        write!(f, "Start Time: {}\n", self.start_time)?;
        write!(f, "Game Ended: {}\n", self.game_ended)?;
        write!(f, "Last Chunk ID: {}\n", self.last_chunk_id)?;
        write!(f, "Last Key Frame ID: {}\n", self.last_key_frame_id)?;
        write!(f, "End Startup Chunk ID: {}\n", self.end_startup_chunk_id)?;
        write!(f, "Delay Time: {}\n", self.delay_time)?;
        write!(
            f,
            "Key Frame Time Interval: {}\n",
            self.key_frame_time_interval
        )?;
        write!(
            f,
            "Decoded Encryption Key: {}\n",
            self.decoded_encryption_key
        )?;
        write!(f, "Start Game Chunk ID: {}\n", self.start_game_chunk_id)?;
        write!(f, "Game Length: {}\n", self.game_length)?;
        write!(f, "Client Added Lag: {}\n", self.client_added_lag)?;
        write!(
            f,
            "Client Back Fetching Enabled: {}\n",
            self.client_back_fetching_enabled
        )?;
        write!(
            f,
            "Client Back Fetching Freq: {}\n",
            self.client_back_fetching_freq
        )?;
        write!(f, "Interest Score: {}\n", self.interest_score)?;
        write!(f, "Featured Game: {}\n", self.featured_game)?;
        write!(f, "Create Time: {}\n", self.create_time)?;
        write!(f, "End Game Chunk ID: {}\n", self.end_game_chunk_id)?;
        write!(f, "End Game Key Frame ID: {}\n", self.end_game_key_frame_id)?;

        write!(f, "Pending Available Chunk Info:\n")?;
        for chunk_info in &self.pending_available_chunk_info {
            write!(
                f,
                "\tChunk ID: {}, Duration: {}, Received Time: {}\n",
                chunk_info.chunk_id, chunk_info.duration, chunk_info.received_time
            )?;
        }

        write!(f, "Pending Available Key Frame Info:\n")?;
        for key_frame_info in &self.pending_available_key_frame_info {
            write!(
                f,
                "\tKey Frame ID: {}, Received Time: {}, Next Chunk ID: {}\n",
                key_frame_info.key_frame_id,
                key_frame_info.received_time,
                key_frame_info.next_chunk_id
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LastChunkInfo {
    chunk_id: u32,
    available_since: u64,
    next_available_chunk: u32,
    key_frame_id: u32,
    next_chunk_id: u32,
    end_startup_chunk_id: u32,
    start_game_chunk_id: u32,
    end_game_chunk_id: u32,
    duration: u32,
}

impl fmt::Display for LastChunkInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk ID: {}\n", self.chunk_id)?;
        write!(f, "Available Since: {}\n", self.available_since)?;
        write!(f, "Next Available Chunk: {}\n", self.next_available_chunk)?;
        write!(f, "Key Frame ID: {}\n", self.key_frame_id)?;
        write!(f, "Next Chunk ID: {}\n", self.next_chunk_id)?;
        write!(f, "End Startup Chunk ID: {}\n", self.end_startup_chunk_id)?;
        write!(f, "Start Game Chunk ID: {}\n", self.start_game_chunk_id)?;
        write!(f, "End Game Chunk ID: {}\n", self.end_game_chunk_id)?;
        write!(f, "Duration: {}\n", self.duration)?;
        Ok(())
    }
}

pub async fn fetch_api_version(region: &Region) -> Result<String, reqwest::Error> {
    let base = base_url(region);
    let url = format!("{}/observer-mode/rest/consumer/version", base);

    debug!("Fetching API version from URL: {}", url);

    let response: String = reqwest::get(&url).await?.text().await?;

    debug!("Received API version response: {}", response);
    Ok(response.to_string())
}

pub async fn fetch_game_meta_data(
    region: &Region,
    game_id: &str,
) -> Result<GameMetaData, reqwest::Error> {
    let base = base_url(region);
    let platform_id = region.platform_id();
    let url = format!(
        "{base}/observer-mode/rest/consumer/getGameMetaData/{platform_id}/{game_id}/token",
        base = base,
        platform_id = platform_id,
        game_id = game_id
    );
    debug!("Fetching API game meta data from URL: {}", url);

    let response: GameMetaData = reqwest::get(&url).await?.json().await?;

    debug!("Received API game meta data response: {}", response);
    Ok(response)
}

pub async fn fetch_last_chunk_info(
    region: &Region,
    game_id: &str,
) -> Result<LastChunkInfo, reqwest::Error> {
    let base = base_url(region);
    let platform_id = region.platform_id();
    let url = format!(
        "{base}/observer-mode/rest/consumer/getLastChunkInfo/{platform_id}/{game_id}/0/token",
        base = base,
        platform_id = platform_id,
        game_id = game_id
    );
    debug!("Fetching API last chunk info data from URL: {}", url);

    let response: LastChunkInfo = reqwest::get(&url).await?.json().await?;

    debug!("Received API last chunk info response: {:?}", response);

    Ok(response)
}

pub async fn fetch_game_data_chunk(
    region: &Region,
    game_id: &str,
    chunk_id: &str
    ) -> Result<Vec<u8>, reqwest::Error> {
    let base = base_url(region);
    let platform_id = region.platform_id();
    let url = format!(
        "{base}/observer-mode/rest/consumer/getGameDataChunk/{platform_id}/{game_id}/{chunk_id}/token",
        base = base,
        platform_id = platform_id,
        game_id = game_id,
        chunk_id = chunk_id
    );
    debug!("Fetching API game data chunk from URL: {}", url);
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

pub async fn fetch_keyframe(
    region: &Region,
    game_id: &str,
    keyframe_id: &str
    ) -> Result<Vec<u8>, reqwest::Error> {
    let base = base_url(region);
    let platform_id = region.platform_id();
    let url = format!(
        "{base}/observer-mode/rest/consumer/getKeyFrame/{platform_id}/{game_id}/{keyframe_id}/token",
        base = base,
        platform_id = platform_id,
        game_id = game_id,
        keyframe_id = keyframe_id
    );
    debug!("Fetching API keyframe from URL: {}", url);
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

# TODO write endOfGameStats and featured endpoints

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_fetch_api_version() {
        let mut server = Server::new_async().await;
        let _m = server
            .mock("GET", "/observer-mode/rest/consumer/version")
            .with_body("2.0.0")
            .create();

        let version = fetch_api_version(&Region::KR).await.unwrap();
        assert_eq!(version, "2.0.0");
    }

    #[tokio::test]
    async fn test_fetch_game_meta_data() {
        let mut server = Server::new_async().await;
        let _m = server
            .mock(
                "GET",
                "/observer-mode/rest/consumer/getGameMetaData/KR/6654667050/token",
            )
            .with_status(200)
            .with_body(
                r#"
                {
                    "gameKey": { "gameId": 6654667050, "platformId": "KR" },
                    "gameServerAddress": "",
                    "port": 0,
                    "encryptionKey": "",
                    "chunkTimeInterval": 30000,
                    "startTime": "Aug 15, 2023 8:01:42 PM"
                }
            "#,
            )
            .create();

        let result = fetch_game_meta_data(&Region::KR, "6654667050").await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.game_key.game_id, 6654667050);
        assert_eq!(response.game_key.platform_id, "KR");
    }

    #[tokio::test]
    async fn test_fetch_last_chunk_info() {
        let mut server = Server::new_async().await;
        let _m = server.mock("GET", "/observer-mode/rest/consumer/getLastChunkInfo/KR/6654667050/0/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"chunkId":53,"availableSince":4197815,"nextAvailableChunk":0,"keyFrameId":26,"nextChunkId":53,"endStartupChunkId":1,"startGameChunkId":2,"endGameChunkId":53,"duration":18869}"#)
            .create();

        let result = fetch_last_chunk_info(&Region::KR, "6654667050").await;
        assert!(result.is_ok());

        let chunk_info = result.unwrap();
        assert_eq!(chunk_info.chunk_id, 53);
        assert_eq!(chunk_info.next_available_chunk, 0);
    }

    #[tokio::test]
    async fn test_fetch_game_data_chunk() {
        let mut server = Server::new_async().await;
        let mock_data = b"mocked binary data";
        let _m = server.mock("GET", "/observer-mode/rest/consumer/getGameDataChunk/KR/6654667050/1/token")
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(mock_data)
            .create();

        let result = fetch_game_data_chunk(&Region::KR, "6654667050", "1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_keyframe() {
        let mut server = Server::new_async().await;
        let mock_data = b"mocked binary data";
        let _m = server.mock("GET", "/observer-mode/rest/consumer/getKeyFrame/KR/6654667050/1/token")
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(mock_data)
            .create();

        let result = fetch_keyframe(&Region::KR, "6654667050", "1").await;
        assert!(result.is_ok());
    }
}
