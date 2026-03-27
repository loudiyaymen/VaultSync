#![allow(dead_code)]
use ssh2::Session;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
pub fn upload_file_with_retry(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    local_path: &Path,
    remote_path: &Path,
    max_retries: u32,
    backoff_ms: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    for attempt in 1..=max_retries {
        match upload_file(host, port, username, password, local_path, remote_path) {
            Ok(_) => {
                println!("Upload succeeded on attempt {}", attempt);
                return Ok(());
            }
            Err(e) => {
                eprintln!("Upload failed (attempt {}): {}", attempt, e);
                if attempt < max_retries {
                    std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                }
            }
        }
    }

    Err(format!("All {} upload attempts failed", max_retries).into())
}

pub fn upload_file(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    local_path: &Path,
    remote_path: &Path,
) -> anyhow::Result<()> {
    let tcp = TcpStream::connect((host, port))?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;
    session.userauth_password(username, password)?;

    let sftp = session.sftp()?;
    let mut remote_file = sftp.create(remote_path)?;
    let mut local_file = File::open(local_path)?;
    let mut buffer = Vec::new();
    local_file.read_to_end(&mut buffer)?;
    remote_file.write_all(&buffer)?;

    Ok(())
}
