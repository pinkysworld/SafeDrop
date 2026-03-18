use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "safedrop", about = "Home-hosted secure file sharing")]
pub struct Config {
    /// Port to listen on
    #[arg(long, env = "SAFEDROP_PORT", default_value = "8080")]
    pub port: u16,

    /// Bind address
    #[arg(long, env = "SAFEDROP_HOST", default_value = "127.0.0.1")]
    pub host: String,

    /// Data directory for chunks, manifests, and audit logs
    #[arg(long, env = "SAFEDROP_DATA_DIR", default_value = "./safedrop-data")]
    pub data_dir: PathBuf,

    /// Log level
    #[arg(long, env = "SAFEDROP_LOG", default_value = "info")]
    pub log_level: String,
}

impl Config {
    pub fn chunks_dir(&self) -> PathBuf {
        self.data_dir.join("chunks")
    }

    pub fn manifests_dir(&self) -> PathBuf {
        self.data_dir.join("manifests")
    }

    pub fn audit_dir(&self) -> PathBuf {
        self.data_dir.join("audit")
    }

    pub fn shares_dir(&self) -> PathBuf {
        self.data_dir.join("shares")
    }

    pub fn receipts_dir(&self) -> PathBuf {
        self.data_dir.join("receipts")
    }

    pub fn network_dir(&self) -> PathBuf {
        self.data_dir.join("network")
    }
}
