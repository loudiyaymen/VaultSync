use serde::Deserialize;
use std::{fs, path::Path, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub customer: String,
    pub template_type: String,
    pub template_code: String,
    pub company: String,
    pub repository: String,
    pub encrypted_output_dir: PathBuf,
    pub sftp_host: String,
    pub sftp_port: u16,
    pub sftp_user: String,
    pub sftp_pass: String,
    pub sftp_remote_dir: String,
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub public_key_path: PathBuf,
    pub encrypted_dir: PathBuf,
    pub pgp_public_key_path: PathBuf,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}
