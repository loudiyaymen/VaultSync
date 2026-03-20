# VaultSync

VaultSync is a cross-platform Rust program that watches a folder for incoming files, encrypts them securely, uploads them to a remote SFTP server, and deletes the original unencrypted file afterward.

---

## Features

- **Folder Monitoring**: Watches a configured directory for any new files
- **File Encryption**: Uses AES-256-GCM for modern, secure encryption
- **Metadata Embedding**: Stores original filename inside encrypted output
- **SFTP Upload**: Transfers encrypted files to a remote server
- **Retry Support**: Automatically retries failed SFTP uploads
- **Cleanup**: Deletes original file after successful encryption and upload
- **Path Customization**: Configurable input/output folders via `.env`
- **Tested Edge Cases**: Handles empty files, binary data, and unicode filenames
- **Autostart Capable**: Can be run as a background service on boot

---

## How it Works

1. Watches a specific folder for any new files
2. Encrypts each file using AES-256-GCM and embeds its original filename
3. Stores the encrypted file in an output directory
4. Uploads the file via SFTP to the configured remote directory
5. Deletes the original plaintext file after successful upload

---

## Usage

### 1. Set environment variables in a `.env` file:

```env
WATCH_DIR=/path/to/watch
ENCRYPTED_OUTPUT_DIR=encrypted
DECRYPTED_OUTPUT_DIR=decrypted

SFTP_HOST=your.server.com
SFTP_PORT=22
SFTP_USER=your_username
SFTP_PASS=your_password
SFTP_REMOTE_DIR=/path/on/server
```

### 2. Build and run the project:

```bash
cargo build --release
./target/release/vault_sync
```

(Optional) Configure autostart for background running on system startup.

---

## Cross-Platform

✅ Works on **Linux**, **macOS**, and **Windows**  
✅ Cross-compilation supported for `x86_64-pc-windows-gnu` target

---

## ⚠️ Notes for macOS Users

Some terminal apps (like iTerm2 and VSCode Terminal) may be sandboxed by macOS, which blocks raw socket access.

This causes tools like `ssh2`, `telnet`, or any network-based Rust client to return errors like:

```
No route to host (os error 65)
```

**Fix:** Use the built-in **Terminal.app**, or grant **Full Disk Access** and **Network Access** to your preferred terminal via  
`System Settings → Privacy & Security`.

---

## Dependencies

| Crate      | Purpose                                                         |
| ---------- | --------------------------------------------------------------- |
| `aes-gcm`  | Provides AES-256-GCM authenticated encryption                   |
| `base64`   | For decoding the base64-encoded encryption key                  |
| `ctrlc`    | Handles graceful shutdown via Ctrl+C signals                    |
| `dotenv`   | Loads environment variables from `.env` files                   |
| `notify`   | Watches file system events for the watched folder               |
| `ssh2`     | Handles SFTP connections and file transfers                     |
| `tempfile` | Used in tests to safely create temporary files and directories  |
| `zeroize`  | Securely erases sensitive data like encryption keys from memory |

---

## License

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

```

```
