use aes_gcm::{
    self,
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit,
};
use std::{fs, path::Path};
pub fn encrypt_file(path: &str, output: &str, key: &Key<Aes256Gcm>) -> std::io::Result<()> {
    // let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let file_bytes = fs::read(Path::new(path))?;
    let cipher_text = cipher
        .encrypt(&nonce, file_bytes.as_ref())
        .expect("Encrytpion failed");
    write_encrypted_file(output, &nonce, &cipher_text)
}
fn write_encrypted_file(output_path: &str, nonce: &[u8], ciphertext: &[u8]) -> std::io::Result<()> {
    let mut buffer = Vec::with_capacity(nonce.len() + ciphertext.len());
    buffer.extend_from_slice(nonce);
    buffer.extend_from_slice(ciphertext);
    fs::write(output_path, buffer)
}
