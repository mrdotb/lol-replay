use super::models::Record;

use std::fs;
use std::io;
use std::path::PathBuf;

pub trait Storage {
    fn store_game_data_chunk(&self, chunk_id: u32, data: Vec<u8>) -> Result<(), io::Error>;
    fn store_key_frame(&self, frame_id: u32, data: Vec<u8>) -> Result<(), io::Error>;
}

pub struct DiskStorage {
    base_path: PathBuf,
}

impl DiskStorage {
    pub fn new(base_path: PathBuf) -> Result<Self, io::Error> {
        fs::create_dir_all(base_path.join("game_data_chunks"))?;
        fs::create_dir(base_path.join("keyframes"))?;
        Ok(DiskStorage { base_path })
    }
}

impl Storage for DiskStorage {
    fn store_game_data_chunk(&self, chunk_id: u32, data: Vec<u8>) -> Result<(), io::Error> {
        let path = self
            .base_path
            .join(format!("game_data_chunks/{}", chunk_id));
        std::fs::write(path, data)
    }

    fn store_key_frame(&self, keyframe_id: u32, data: Vec<u8>) -> Result<(), io::Error> {
        let path = self.base_path.join(format!("keyframes/{}", keyframe_id));
        std::fs::write(path, data)
    }
}
