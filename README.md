# SecureDrop

SecureDrop is a cross-platform Rust program that watches a folder for incoming files, encrypts them securely, uploads them to a remote SFTP server, and deletes the original unencrypted file afterward.

## Features

- 📁 **Folder Monitoring**: Watches for any new file in a designated directory
- 🔐 **File Encryption**: Uses AES-256-GCM encryption for high-security standards
- 🌐 **SFTP Upload**: Sends encrypted files to a remote server over a secure SFTP connection
- 🧹 **Cleanup**: Deletes original plaintext files after secure upload
- 🔁 **Autostart Capable**: Can be configured to run automatically at system boot on Windows

## How it works

1. Watches a specific folder for any new files
2. Encrypts each file using a pre-shared key
3. Sends the encrypted file via SFTP
4. Keeps the encrypted version, deletes the original

## Usage

1. Set configuration values in a `.env` file:

   ```env
   WATCH_DIR=/path/to/watch
   ENCRYPTION_KEY=base64encodedkey==
   SFTP_HOST=sftp.example.com
   SFTP_USERNAME=your_username
   SFTP_PASSWORD=your_password
   SFTP_DEST_DIR=/remote/path

    Build and run the project:

    cargo build --release
    ./target/release/securedrop

    (Optional) Configure autostart for background running on system startup.
   ```

Cross-Platform

    ✅ Works on Linux, macOS, and Windows

    ✅ Cross-compilation supported (x86_64-pc-windows-gnu for Windows binaries)

Dependencies

    notify — file system watching

    aes-gcm — encryption

    ssh2 — SFTP support

    dotenv — environment variable loading

License

MIT License
