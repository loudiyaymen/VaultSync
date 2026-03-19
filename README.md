# VaultSync

VaultSync is a cross-platform Rust program that watches a folder for incoming files, encrypts them securely, uploads them to a remote SFTP server, and deletes the original unencrypted file afterward.

## Features

- **Folder Monitoring**: Watches for any new file in a designated directory
- **File Encryption**: Uses AES-256-GCM encryption for high-security standards
- **SFTP Upload**: Sends encrypted files to a remote server over a secure SFTP connection
- **Cleanup**: Deletes original plaintext files after secure upload
- **Autostart Capable**: Can be configured to run automatically at system boot on Windows

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
    ./target/release/vault_sync

    (Optional) Configure autostart for background running on system startup.
   ```

Cross-Platform

    ✅ Works on Linux, macOS, and Windows

    ✅ Cross-compilation supported (x86_64-pc-windows-gnu for Windows binaries)

⚠️ Important Notes for macOS Users

    Terminal apps like iTerm2 and VSCode Terminal may be sandboxed by macOS.

    Running ssh2, telnet, or any raw socket-based Rust code from those apps may result in “No route to host” or similar errors.

    To avoid this: run from the default macOS Terminal.app or ensure your terminal app is granted full disk and network permissions under System Settings → Privacy & Security.

Dependencies

    notify — file system watching

    aes-gcm — encryption

    ssh2 — SFTP support

    dotenv — environment variable loading

License

MIT License

# VaultSync TODO List

## Setup

- [x] Initialize project with `cargo init`
- [x] Set up `.gitignore`
- [x] Add dependencies: `aes-gcm`, `rand`, `notify`, `dotenv`, `ssh2`, `zeroize`
- [x] Load `.env` file using `dotenv` in `config.rs`
- [x] Read `WATCH_DIR` environment variable from `.env`

## Core Features

- [x] Implement folder watcher using `notify` crate (`watcher.rs`)
- [x] Spawn watcher on a separate thread
- [x] Dynamically load `WATCH_DIR` and validate existence
- [x] Log raw filesystem events for visibility
- [x] Handle `Create(File)` and `Modify(Name)` events
- [x] Implement file encryption using AES-256-GCM (`encryptor.rs`)
- [x] Generate secure random nonce per file
- [x] Prepend nonce to ciphertext in output file
- [x] Embed original filename or extension as metadata in encrypted output
- [x] Write encrypted output with `.vault` extension (based on file stem)
- [x] Automatically append `.vault` extension, removing original
- [x] Upload encrypted file via SFTP (`sftp.rs`)
- [x] Delete original file after encryption and upload
- [x] Wire up `config`, `encryptor`, `sftp`, and `watcher` in `main.rs`
- [x] Gracefully shut down watcher using `ctrlc` signal
- [x] Securely zeroize encryption key after use

## Testing

- [x] Write unit tests for `encryptor` with roundtrip validation
- [ ] Write unit tests for `config`, and `watcher` modules
- [ ] Test edge cases (empty file, binary, unicode filename)

## Code Improvements

- [x] Refactor decrypt logic to read and apply original filename
- [x] Replace unwrap/expect with proper error handling in critical paths
- [x] Refactor filename handling using `Path::with_file_name` and `.join()`
- [ ] Refactor shared I/O utilities into helpers
- [ ] Consider splitting `encryptor.rs` into `encryptor.rs` and `decryptor.rs` for clarity
- [ ] Add retry/backoff logic to failed SFTP uploads

## Operational

- [x] Warn macOS users: `telnet`, `ssh2`, and SFTP may not work inside VSCode/iTerm2 — use macOS Terminal instead
- [ ] Add optional logging toggle or log levels via `.env`
- [ ] Create system service or background runner for autostart (Windows/Unix)

## Future

- [ ] Support decryption CLI for restoring `.vault` files
- [ ] Optionally archive encrypted files instead of deleting originals
- [ ] Implement file size limit / throttling for large files
- [ ] Add UI or tray monitor for running in background
