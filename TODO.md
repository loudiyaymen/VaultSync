# VaultSync TODO List

## Setup

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
- [x] Wire up `config`, `encryptor`, and `watcher` in `main.rs`
- [x] Handle Result types properly in `encrypt_file`

## Testing

- [ ] Write unit tests for `config`, `encryptor`, and `watcher` modules
- [ ] Test `encrypt_file` cleanly using temporary files

## Code Improvements

- [ ] Replace remaining `unwrap`/`expect` with proper error handling
- [ ] Automatically append `.vault` extension if missing

## Features

- [ ] Implement `decrypt_file` to support full encryption ➔ decryption roundtrip

## Operational (Next)

- [ ] Handle graceful shutdown (stop watcher thread on Ctrl+C)
- [ ] Securely zeroize encryption keys after use
