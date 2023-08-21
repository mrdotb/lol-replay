use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;

pub trait Storage {
    fn store_game_data_chunk(&self, chunk_id: u32, data: Vec<u8>) -> Result<(), io::Error>;
    fn store_key_frame(&self, frame_id: u32, data: Vec<u8>) -> Result<(), io::Error>;

    fn metadata(&self) -> String;
}

pub struct DiskStorage {
    base_path: PathBuf,
}

impl DiskStorage {
    pub fn new(base_path: PathBuf) -> Result<Self, io::Error> {
        Self::create_dir_if_not_exists(base_path.join("game_data_chunks"))?;
        Self::create_dir_if_not_exists(base_path.join("keyframes"))?;
        Ok(DiskStorage { base_path })
    }

    fn create_dir_if_not_exists(path: PathBuf) -> Result<(), io::Error> {
        match fs::create_dir_all(&path) {
            Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()), // If it already exists, just return Ok
            other => other,
        }
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

    fn metadata(&self) -> String {
        format!("DiskStorage: base_path: {:?}", self.base_path)
    }
}
