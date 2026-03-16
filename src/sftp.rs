use ssh2::Session;
use std::{
    env,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

pub fn upload_file(local_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let host = env::var("SFTP_HOST")?;
    let port = env::var("SFTP_PORT").unwrap_or_else(|_| "22".to_string());
    let username = env::var("SFTP_USER")?;
    let password = env::var("SFTP_PASS")?;
    let remote_dir = env::var("SFTP_REMOTE_DIR")?;

    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(&username, &password)?;
    assert!(sess.authenticated());

    let sftp = sess.sftp()?;

    let local_file = Path::new(local_path);
    let filename = local_file.file_name().unwrap().to_str().unwrap();
    let remote_path = Path::new(&remote_dir).join(filename);

    let mut local = File::open(local_file)?;
    let mut remote = sftp.create(&remote_path)?;
    let mut buffer = Vec::new();
    local.read_to_end(&mut buffer)?;
    remote.write_all(&buffer)?;

    println!("Uploaded {} to {}", filename, remote_path.display());

    Ok(())
}
