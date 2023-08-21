use crate::api::models::{GameMetaData, SpectatorEndpoint};

use super::storage::Storage;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;

pub struct Record {
    pub version: String,
    pub endpoint: SpectatorEndpoint,
    pub game_id: String,
    pub encryption_key: String,
    pub metadata: Option<GameMetaData>,
    pub keyframes: HashSet<u32>,
    pub game_data_chunks: HashSet<u32>,
    pub storage: Box<dyn Storage>,
}

impl Record {
    pub fn new(
        version: String,
        endpoint: SpectatorEndpoint,
        game_id: String,
        encryption_key: String,
        storage: Box<dyn Storage>,
    ) -> Self {
        Record {
            version,
            endpoint,
            game_id,
            encryption_key,
            metadata: None,
            keyframes: HashSet::new(),
            game_data_chunks: HashSet::new(),
            storage,
        }
    }

    pub fn has_game_data_chunk(&self, chunk_id: u32) -> bool {
        self.game_data_chunks.contains(&chunk_id)
    }

    pub fn insert_game_data_chunk(&mut self, chunk_id: u32) {
        self.game_data_chunks.insert(chunk_id);
    }

    pub fn has_keyframe(&self, chunk_id: u32) -> bool {
        self.keyframes.contains(&chunk_id)
    }

    pub fn insert_keyframe(&mut self, chunk_id: u32) {
        self.keyframes.insert(chunk_id);
    }

    pub fn save_to_file(&self) -> Result<(), io::Error> {
        fs::create_dir_all(format!("./completed/{}", self.endpoint.platform_id))?;
        let filename = format!(
            "./completed/{}/{}.json",
            self.endpoint.platform_id, self.game_id
        );
        let json = serde_json::to_string(&self).unwrap();
        fs::write(filename, json)
    }
}

impl Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Record", 8)?;

        state.serialize_field("version", &self.version)?;
        state.serialize_field("endpoint", &self.endpoint)?;
        state.serialize_field("game_id", &self.game_id)?;
        state.serialize_field("encryption_key", &self.encryption_key)?;
        state.serialize_field("metadata", &self.metadata)?;

        // Sorting keyframes in ascending order
        state.serialize_field("keyframes", &{
            let mut sorted = self.keyframes.iter().cloned().collect::<Vec<_>>();
            sorted.sort();
            sorted
        })?;

        // Sorting game_data_chunks in ascending order
        state.serialize_field("game_data_chunks", &{
            let mut sorted = self.game_data_chunks.iter().cloned().collect::<Vec<_>>();
            sorted.sort();
            sorted
        })?;

        state.serialize_field("storage", &self.storage.metadata())?;
        state.end()
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Version: {}\n", self.version)?;
        write!(f, "Game Id: {}\n", self.game_id)?;
        write!(f, "Encryption Key: {}\n", self.game_id)?;

        Ok(())
    }
}
