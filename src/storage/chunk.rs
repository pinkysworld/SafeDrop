//! Content-addressed chunk store with BLAKE3 hashing.

use crate::error::AppError;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Fixed chunk size: 1 MiB
pub const CHUNK_SIZE: usize = 1024 * 1024;

/// Content-addressed chunk store.
#[derive(Clone)]
pub struct ChunkStore {
    root: PathBuf,
}

impl ChunkStore {
    pub async fn new(root: PathBuf) -> Result<Self, AppError> {
        fs::create_dir_all(&root).await?;
        Ok(Self { root })
    }

    /// Store a chunk. Returns the BLAKE3 hash as hex.
    pub async fn put(&self, data: &[u8]) -> Result<String, AppError> {
        let hash = blake3::hash(data).to_hex().to_string();
        let path = self.chunk_path(&hash);

        // Content-addressed: skip if already stored
        if path.exists() {
            return Ok(hash);
        }

        // Write atomically: write to temp, then rename
        let tmp_path = self.root.join(format!(".tmp-{}", &hash[..16]));
        let mut file = fs::File::create(&tmp_path).await?;
        file.write_all(data).await?;
        file.flush().await?;
        file.sync_all().await?;

        fs::rename(&tmp_path, &path).await?;

        tracing::debug!(hash = %hash, size = data.len(), "chunk stored");
        Ok(hash)
    }

    /// Retrieve chunk data by hash.
    pub async fn get(&self, hash: &str) -> Result<Vec<u8>, AppError> {
        let path = self.chunk_path(hash);
        if !path.exists() {
            return Err(AppError::NotFound(format!("chunk {hash}")));
        }
        let data = fs::read(&path).await?;

        // Verify integrity on read
        let actual = blake3::hash(&data).to_hex().to_string();
        if actual != hash {
            return Err(AppError::Internal(format!(
                "chunk integrity failure: expected {hash}, got {actual}"
            )));
        }
        Ok(data)
    }

    /// Check if a chunk exists.
    pub async fn exists(&self, hash: &str) -> bool {
        self.chunk_path(hash).exists()
    }

    fn chunk_path(&self, hash: &str) -> PathBuf {
        // Two-level directory structure: ab/cdef...
        let (prefix, rest) = hash.split_at(2.min(hash.len()));
        self.root.join(prefix).join(rest)
    }

    /// Ensure the prefix directory exists, then return the path.
    async fn ensure_chunk_dir(&self, hash: &str) -> Result<PathBuf, AppError> {
        let path = self.chunk_path(hash);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        Ok(path)
    }

    /// Store a chunk with directory creation. Returns the BLAKE3 hash.
    pub async fn put_with_dirs(&self, data: &[u8]) -> Result<String, AppError> {
        let hash = blake3::hash(data).to_hex().to_string();
        let path = self.ensure_chunk_dir(&hash).await?;

        if path.exists() {
            return Ok(hash);
        }

        let tmp_path = self.root.join(format!(".tmp-{}", &hash[..16]));
        let mut file = fs::File::create(&tmp_path).await?;
        file.write_all(data).await?;
        file.flush().await?;
        file.sync_all().await?;

        fs::rename(&tmp_path, &path).await?;

        tracing::debug!(hash = %hash, size = data.len(), "chunk stored");
        Ok(hash)
    }
}
