use aes_gcm::{
    self,
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

pub fn encrypt_file(path: &str, output: &str, key: &Key<Aes256Gcm>) -> std::io::Result<()> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let file_bytes = fs::read(Path::new(path))?;

    let cipher_text = cipher
        .encrypt(&nonce, file_bytes.as_ref())
        .expect("Encryption failed");

    let filename = Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .as_bytes();

    let filename_len: u16 = filename.len().try_into().expect("Filename too long");
    let filename_len_bytes = filename_len.to_be_bytes();

    let mut buffer = Vec::with_capacity(12 + 2 + filename.len() + cipher_text.len());
    buffer.extend_from_slice(&nonce);
    buffer.extend_from_slice(&filename_len_bytes);
    buffer.extend_from_slice(filename);
    buffer.extend_from_slice(&cipher_text);

    fs::write(output, buffer)
}

pub fn decrypt_file(path: &str, key: &Key<Aes256Gcm>) {
    let mut file = File::open(path).expect("Failed to open encrypted file");
    let mut nonce_bytes = [0u8; 12];
    file.read_exact(&mut nonce_bytes)
        .expect("Failed to read nonce");

    let mut len_bytes = [0u8; 2];
    file.read_exact(&mut len_bytes)
        .expect("Failed to read filename length");
    let filename_len = u16::from_be_bytes(len_bytes) as usize;

    let mut filename_bytes = vec![0u8; filename_len];
    file.read_exact(&mut filename_bytes)
        .expect("Failed to read filename");

    let output_name = String::from_utf8(filename_bytes).expect("Invalid UTF-8 in filename");

    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)
        .expect("Failed to read ciphertext");

    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("Decryption failed");
    let output_path = Path::new("decrypted").join(&output_name);
    fs::create_dir_all("decrypted").expect("Failed to create decrypted folder");
    fs::write(&output_path, plaintext).expect("Failed to write decrypted output");
}
