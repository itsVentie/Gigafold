use serde::{Deserialize, Serialize};
use sled::{Db, Tree};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FileType {
    File,
    Directory,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMetadata {
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub chunks: Vec<ChunkInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkInfo {
    pub hash: String,
    pub nonce: [u8; 19],
    pub size: u32,
}

pub struct IndexManager {
    db: Db,
    metadata_tree: Tree,
}

impl IndexManager {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let db = sled::open(path).map_err(|e| e.to_string())?;
        let metadata_tree = db.open_tree("metadata").map_err(|e| e.to_string())?;
        
        Ok(IndexManager { db, metadata_tree })
    }

    pub fn save_file(&self, path: &str, meta: &FileMetadata) -> Result<(), String> {
        let serialized = bincode::serialize(meta).map_err(|e| e.to_string())?;
        self.metadata_tree
            .insert(path.as_bytes(), serialized)
            .map_err(|e| e.to_string())?;
        
        self.db.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_file(&self, path: &str) -> Result<Option<FileMetadata>, String> {
        let res = self.metadata_tree.get(path.as_bytes()).map_err(|e| e.to_string())?;
        
        match res {
            Some(ivec) => {
                let meta: FileMetadata = bincode::deserialize(&ivec).map_err(|e| e.to_string())?;
                Ok(Some(meta))
            }
            None => Ok(None),
        }
    }

    pub fn delete_file(&self, path: &str) -> Result<(), String> {
        self.metadata_tree.remove(path.as_bytes()).map_err(|e| e.to_string())?;
        self.db.flush().map_err(|e| e.to_string())?;
        Ok(())
    }
}