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

## VaultSync TODO List

### Setup

- [x] Initialize project with `cargo init`
- [x] Set up `.gitignore`
- [x] Add dependencies: `aes-gcm`, `rand`, `notify`, `dotenv`
- [x] Load `.env` file using `dotenv` in `config.rs`
- [x] Read `WATCH_DIR` environment variable from `.env`

## Core Features

- [x] Implement folder watcher using `notify` crate (`watcher.rs`)
- [x] Spawn watcher on a separate thread
- [x] Implement file encryption using AES-256-GCM (`encryptor.rs`)
- [x] Generate secure random nonce per file
- [x] Prepend nonce to ciphertext in output file
- [x] Write encrypted file to disk
- [x] Automatically append `.vault` extension, removing original
- [x] Embed original filename or extension as metadata in encrypted output
- [x] Wire up `config`, `encryptor`, and `watcher` in `main.rs`
- [x] Handle Result types properly in `encrypt_file`
- [x] Implement `decrypt_file` to support full encryption ➔ decryption roundtrip
- [x] Write decrypted output to `decrypted/` folder using `Path::new().join()`
- [x] Refactor decrypt logic to read and apply original filename

## Testing

- [x] Write unit tests for `encryptor` with roundtrip validation
- [ ] Write unit tests for `config`, and `watcher` modules
- [ ] Test edge cases (empty file, binary, unicode filename)

## Code Improvements

- [ ] Replace remaining `unwrap`/`expect` with proper error handling
- [ ] Refactor shared I/O utilities into helpers
- [ ] Consider splitting `encryptor.rs` into `encryptor.rs` and `decryptor.rs` for clarity

## Operational

- [x] Handle graceful shutdown (stop watcher thread on Ctrl+C using `ctrlc`)
- [ ] Securely zeroize encryption keys after use

## Future

- [x] Integrate SFTP upload after encryption
- [x] Store SFTP credentials via config/env
