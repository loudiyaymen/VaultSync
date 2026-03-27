# VaultSync

VaultSync is a cross-platform Rust program that watches a folder for incoming files, encrypts them using OpenPGP, uploads them to a remote SFTP server, and deletes the original unencrypted file afterward.

---

## Features

- **Folder Monitoring**: Watches a configured directory for any new files
- **PGP Encryption**: Uses OpenPGP (via Sequoia) to encrypt files using a public key
- **SFTP Upload**: Transfers encrypted files to a remote server securely
- **Retry Support**: Automatically retries failed SFTP uploads with exponential backoff
- **Cleanup**: Deletes the original file after successful encryption and upload
- **Path Customization**: Configurable input/output folders via `.env`
- **Edge Case Handling**: Tested with empty files, binary content, and unicode filenames
- **Autostart Capable**: Can be run as a background service on boot

---

## How it Works

1. Watches a specified folder (`WATCH_DIR`) for any new files
2. Encrypts each file using the configured OpenPGP public key
3. Saves the `.pgp` encrypted file to the `ENCRYPTED_DIR`
4. Uploads the encrypted file via SFTP to the configured remote path
5. Deletes the original plaintext file on success

---

## Usage

### 1. Set environment variables in a `.env` file:

```env
WATCH_DIR=./test
ENCRYPTED_DIR=./encrypted
DECRYPTED_DIR=./decrypted

PGP_PUBLIC_KEY=./keys/recipient.asc
PGP_PRIVATE_KEY=./keys/secret.asc # (optional, not yet used)

SFTP_HOST=your.server.com
SFTP_PORT=22
SFTP_USER=your_username
SFTP_PASS=your_password
SFTP_REMOTE_DIR=/path/on/server

SFTP_RETRY=3
SFTP_RETRY_BACKOFF_MS=1000
```

### 2. Build and run:

```bash
cargo build --release
./target/release/vault_sync
```

(Optional) Set up system service to run VaultSync automatically on startup.

---

## Cross-Platform

✅ Works on **Linux**, **macOS**, and **Windows**  
✅ Cross-compilation supported for `x86_64-pc-windows-gnu`

---

## ⚠️ Notes for macOS Users

Some terminal apps (e.g., iTerm2, VSCode Terminal) are sandboxed and may prevent SSH/SFTP network access.

You may encounter errors like:

```
No route to host (os error 65)
```

**Fix:** Use the default **Terminal.app**, or grant full disk and network access under:

> System Settings → Privacy & Security

---

## Dependencies

| Crate             | Purpose                                                |
| ----------------- | ------------------------------------------------------ |
| `sequoia-openpgp` | Handles OpenPGP-based encryption                       |
| `aes-gcm`         | (Deprecated in current branch) Previously used for AES |
| `base64`          | Used for key decoding (AES legacy support)             |
| `ctrlc`           | Handles Ctrl+C graceful shutdown                       |
| `dotenv`          | Loads configuration from `.env`                        |
| `notify`          | Watches file system changes                            |
| `ssh2`            | SFTP connection and upload                             |
| `tempfile`        | Creates temp files during testing                      |
| `zeroize`         | Securely erases keys from memory                       |

---

## License

MIT License

---

## VaultSync TODO List

### Setup

- [x] Initialize project with `cargo init`
- [x] Add `.gitignore` and `.env`
- [x] Configure `.env` loading
- [x] Set up folder watching and configuration

### Core Features

- [x] Folder watcher using `notify`
- [x] Graceful shutdown with `ctrlc`
- [x] Load OpenPGP public key from path (`PGP_PUBLIC_KEY`)
- [x] Encrypt incoming files with `sequoia-openpgp`
- [x] Output `.pgp` files to `ENCRYPTED_DIR`
- [x] Upload encrypted files via SFTP with retry
- [x] Delete original files after success

### Testing

- [x] Roundtrip tests (AES branch only)
- [x] Add tests for PGP encryption and output
- [x] Test empty, binary, and unicode filenames
- [ ] Add integration test to simulate file watch + encryption + upload

### Improvements

- [x] Refactor watcher to skip already encrypted files
- [x] Use config for all paths and retry settings
- [ ] Optional logging toggle and verbosity level
- [ ] UI or tray agent (future)

### Future

- [ ] Add decryption support using `PGP_PRIVATE_KEY`
- [ ] Archive encrypted files post-upload instead of deleting
- [ ] Support signing files with private key
- [ ] Add encryption method switch (AES <-> PGP)
