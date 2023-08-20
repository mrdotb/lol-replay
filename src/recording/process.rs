use crate::api::endpoints;
use crate::api::models::SpectatorEndpoint;

use super::models::Record;
use super::storage::Storage;

use log::debug;
use std::thread::sleep;
use std::time::Duration;

pub async fn new(
    endpoint: SpectatorEndpoint,
    game_id: String,
    encryption_key: String,
    storage: Box<dyn Storage>,
) -> Result<(), reqwest::Error> {
    let version = endpoints::fetch_api_version(&endpoint).await?;
    let mut record = Record::new(version, endpoint, game_id, encryption_key, storage);

    let metadata = endpoints::fetch_game_meta_data(&record.endpoint, &record.game_id).await?;
    record.metadata = Some(metadata);

    let _chunk_info = endpoints::fetch_last_chunk_info(&record.endpoint, &record.game_id).await?;

    record_media_data(&record).await?;

    Ok(())
}

async fn record_media_data(record: &Record) -> Result<(), reqwest::Error> {
    let mut first_chunk_info = true;

    loop {
        match endpoints::fetch_last_chunk_info(&record.endpoint, &record.game_id).await {
            Ok(chunk_info) => {
                if first_chunk_info && chunk_info.chunk_id != 1 && chunk_info.key_frame_id != 1 {
                    debug!("Received first chunk info but it's not the first one try to download previous media data");
                    process_previous_media_data(
                        record,
                        chunk_info.chunk_id,
                        chunk_info.key_frame_id,
                    )
                    .await?;
                    first_chunk_info = false;
                }

                if chunk_info.chunk_id == chunk_info.end_game_chunk_id {
                    debug!("Received last chunk info");
                    break;
                }

                let waiting_time = Duration::from_millis(chunk_info.next_available_chunk as u64)
                    + Duration::from_secs(1);
                debug!("Wait {:?} milliseconds before next iteration", waiting_time);
                sleep(waiting_time);
                debug!("storing chunk id {}", chunk_info.chunk_id);
            }
            Err(error) => {
                debug!(
                    "Record Frames received error {} retry in 10 seconds...",
                    error
                );
                sleep(Duration::from_secs(10));
                continue;
            }
        }
    }
    Ok(())
}

async fn process_previous_media_data(
    record: &Record,
    current_chunk_id: u32,
    current_key_frame_id: u32,
) -> Result<(), reqwest::Error> {
    for chunk_id in (1..=current_chunk_id).rev() {
        match endpoints::fetch_game_data_chunk(&record.endpoint, &record.game_id, chunk_id).await {
            Ok(game_data_chunk) => {
                debug!("Storing game data chunk id {}", chunk_id);
                record
                    .storage
                    .store_game_data_chunk(chunk_id, game_data_chunk);
            }
            Err(error) => {
                debug!("error {}", error);
            }
        }
    }

    for keyframe_id in (1..=current_key_frame_id).rev() {
        match endpoints::fetch_keyframe(&record.endpoint, &record.game_id, keyframe_id).await {
            Ok(keyframe) => {
                debug!("Storing keyframe {}", keyframe_id);
                record.storage.store_key_frame(keyframe_id, keyframe);
            }
            Err(error) => {
                debug!("error {}", error);
            }
        }
    }
    Ok(())
}
