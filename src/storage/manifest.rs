//! Deterministic manifest generation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncReadExt;

use crate::error::AppError;
use crate::storage::chunk::{ChunkStore, CHUNK_SIZE};

/// Schema version for manifests.
pub const MANIFEST_SCHEMA_VERSION: &str = "1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkRef {
    pub index: u32,
    pub hash: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunking {
    pub algorithm: String,
    pub chunk_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub schema_version: String,
    pub manifest_id: String,
    pub created_at: DateTime<Utc>,
    pub content_type: String,
    pub size_bytes: u64,
    pub chunking: Chunking,
    pub chunks: Vec<ChunkRef>,
    pub file_name_alias: String,
    pub hash_algorithm: String,
}

impl Manifest {
    /// Compute the manifest ID deterministically from chunk hashes.
    pub fn compute_id(chunks: &[ChunkRef]) -> String {
        let mut hasher = blake3::Hasher::new();
        for c in chunks {
            hasher.update(c.hash.as_bytes());
        }
        hasher.finalize().to_hex().to_string()
    }
}

/// Ingest a file: chunk it, store chunks, produce a manifest.
pub async fn ingest_file(
    store: &ChunkStore,
    file_path: &std::path::Path,
    file_name: &str,
    content_type: &str,
) -> Result<Manifest, AppError> {
    let mut file = fs::File::open(file_path).await?;
    let metadata = file.metadata().await?;
    let total_size = metadata.len();

    let mut chunks = Vec::new();
    let mut buf = vec![0u8; CHUNK_SIZE];
    let mut index = 0u32;

    loop {
        let mut read_total = 0;
        // Read exactly CHUNK_SIZE or until EOF
        loop {
            let n = file.read(&mut buf[read_total..]).await?;
            if n == 0 {
                break;
            }
            read_total += n;
            if read_total >= CHUNK_SIZE {
                break;
            }
        }

        if read_total == 0 {
            break;
        }

        let chunk_data = &buf[..read_total];
        let hash = store.put_with_dirs(chunk_data).await?;

        chunks.push(ChunkRef {
            index,
            hash,
            size: read_total as u64,
        });
        index += 1;
    }

    let manifest_id = Manifest::compute_id(&chunks);

    let manifest = Manifest {
        schema_version: MANIFEST_SCHEMA_VERSION.to_string(),
        manifest_id,
        created_at: Utc::now(),
        content_type: content_type.to_string(),
        size_bytes: total_size,
        chunking: Chunking {
            algorithm: "fixed-1MiB".to_string(),
            chunk_count: chunks.len() as u32,
        },
        chunks,
        file_name_alias: file_name.to_string(),
        hash_algorithm: "blake3".to_string(),
    };

    tracing::info!(
        manifest_id = %manifest.manifest_id,
        chunks = manifest.chunking.chunk_count,
        size = total_size,
        "manifest created"
    );

    Ok(manifest)
}

/// Store a manifest to disk as JSON.
pub async fn store_manifest(manifests_dir: &PathBuf, manifest: &Manifest) -> Result<(), AppError> {
    fs::create_dir_all(manifests_dir).await?;
    let path = manifests_dir.join(format!("{}.json", &manifest.manifest_id));
    let json = serde_json::to_string_pretty(manifest)?;
    fs::write(&path, json.as_bytes()).await?;
    Ok(())
}

/// Load a manifest from disk.
pub async fn load_manifest(manifests_dir: &PathBuf, manifest_id: &str) -> Result<Manifest, AppError> {
    let path = manifests_dir.join(format!("{manifest_id}.json"));
    if !path.exists() {
        return Err(AppError::NotFound(format!("manifest {manifest_id}")));
    }
    let data = fs::read_to_string(&path).await?;
    let manifest: Manifest = serde_json::from_str(&data)?;
    Ok(manifest)
}
