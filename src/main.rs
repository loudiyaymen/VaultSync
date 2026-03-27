mod config;
mod encryptor;
mod pgp;
mod sftp;

use crate::config::Config;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load_from_file("config.toml")?;

    let input_path = Path::new("test/test.txt");
    let encrypted_filename = input_path.file_name().unwrap().to_str().unwrap().to_owned() + ".asc";

    let output_path = Path::new(&config.encrypted_dir).join(&encrypted_filename);
    let pubkey_path = Path::new(&config.pgp_public_key_path);

    println!("Encrypting...");
    pgp::encrypt_file(pubkey_path, input_path, &output_path)?;
    println!("Encrypted to {}", output_path.display());

    println!("Uploading...");
    sftp::upload_file(
        &config.sftp_host,
        config.sftp_port,
        &config.sftp_user,
        &config.sftp_pass,
        &output_path,
        &Path::new(&config.sftp_remote_dir).join(&encrypted_filename),
    )?;
    println!("Upload successful.");

    Ok(())
}
