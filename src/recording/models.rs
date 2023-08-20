use crate::api::models::{GameMetaData, SpectatorEndpoint};

use super::storage::Storage;

use std::collections::HashSet;
use std::fmt;

pub struct Record {
    pub version: String,
    pub endpoint: SpectatorEndpoint,
    pub game_id: String,
    pub encryption_key: String,
    pub metadata: Option<GameMetaData>,
    pub frames: HashSet<u32>,
    pub chunks: HashSet<u32>,
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
            frames: HashSet::new(),
            chunks: HashSet::new(),
            storage,
        }
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
