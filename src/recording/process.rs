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

    record_media_data(&mut record).await?;

    Ok(())
}

async fn record_media_data(record: &mut Record) -> Result<(), reqwest::Error> {
    let mut current_chunk_id = 1;
    let mut current_keyframe_id = 1;

    loop {
        match endpoints::fetch_last_chunk_info(&record.endpoint, &record.game_id).await {
            Ok(chunk_info) => {
                if chunk_info.chunk_id != current_chunk_id
                    || chunk_info.key_frame_id != current_keyframe_id
                {
                    debug!("Received first chunk info but there is a gap between chunk_id or keyframe_id try to download previous media data");
                    process_previous_media_data(
                        record,
                        chunk_info.chunk_id,
                        chunk_info.key_frame_id,
                    )
                    .await?;
                }

                process_media_data(record, chunk_info.chunk_id, chunk_info.key_frame_id).await?;

                if chunk_info.chunk_id == chunk_info.end_game_chunk_id {
                    debug!("Received last chunk info");
                    record.save_to_file().unwrap();
                    break;
                }

                current_chunk_id += 1;
                current_keyframe_id += 1;

                let waiting_time = Duration::from_millis(chunk_info.next_available_chunk as u64)
                    + Duration::from_secs(1);
                debug!("Wait {:?} milliseconds before next iteration", waiting_time);
                sleep(waiting_time);
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
    record: &mut Record,
    current_chunk_id: u32,
    current_key_frame_id: u32,
) -> Result<(), reqwest::Error> {
    for chunk_id in (1..=current_chunk_id - 1).rev() {
        let _ = fetch_and_store_game_data_chunk(record, chunk_id).await;
    }

    for keyframe_id in (1..=current_key_frame_id - 1).rev() {
        let _ = fetch_and_store_keyframe(record, keyframe_id).await;
    }

    Ok(())
}

async fn process_media_data(
    record: &mut Record,
    chunk_id: u32,
    keyframe_id: u32,
) -> Result<(), reqwest::Error> {
    let _ = fetch_and_store_game_data_chunk(record, chunk_id).await;
    let _ = fetch_and_store_keyframe(record, keyframe_id).await;

    Ok(())
}

async fn fetch_and_store_game_data_chunk(
    record: &mut Record,
    chunk_id: u32,
) -> Result<(), reqwest::Error> {
    // Return if the chunk ID is already in the set
    if record.game_data_chunks.contains(&chunk_id) {
        return Ok(());
    }

    match endpoints::fetch_game_data_chunk(&record.endpoint, &record.game_id, chunk_id).await {
        Ok(game_data_chunk) => {
            debug!("Storing game data chunk id {}", chunk_id);
            if let Err(e) = record
                .storage
                .store_game_data_chunk(chunk_id, game_data_chunk)
            {
                debug!("Error while storing chunk: {}", e);
            } else {
                record.game_data_chunks.insert(chunk_id);
            }
        }
        Err(error) => {
            debug!("error {}", error);
            return Err(error);
        }
    }
    Ok(())
}

async fn fetch_and_store_keyframe(
    record: &mut Record,
    keyframe_id: u32,
) -> Result<(), reqwest::Error> {
    // Return if the keyframe ID is already in the set
    if record.keyframes.contains(&keyframe_id) {
        return Ok(());
    }

    match endpoints::fetch_keyframe(&record.endpoint, &record.game_id, keyframe_id).await {
        Ok(keyframe) => {
            debug!("Storing keyframe {}", keyframe_id);
            if let Err(e) = record.storage.store_key_frame(keyframe_id, keyframe) {
                debug!("Error while storing keyframe: {}", e);
            } else {
                record.keyframes.insert(keyframe_id);
            }
        }
        Err(error) => {
            debug!("error {}", error);
            return Err(error);
        }
    }
    Ok(())
}
