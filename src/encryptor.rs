use aes_gcm::{
    self,
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

pub fn encrypt_file(path: &str, key: &Key<Aes256Gcm>) -> std::io::Result<()> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let file_bytes = fs::read(Path::new(path))?;
    let input_path = Path::new(path);
    let stem = input_path.file_stem().unwrap().to_str().unwrap();
    let output_path = input_path.with_file_name(format!("{}.vault", stem));
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

    fs::write(output_path, buffer)
}

pub fn _decrypt_file(path: &str, key: &Key<Aes256Gcm>) {
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
#[cfg(test)]
mod tests {
    use super::*;
    use aes_gcm::KeyInit;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_encrypt_and_decrypt() {
        let dir = tempdir().expect("failed to create temp dir");

        let input_path = dir.path().join("test.txt");
        let original_content = b"vaultsync test data";
        let mut input_file = File::create(&input_path).expect("failed to create input file");
        input_file
            .write_all(original_content)
            .expect("failed to write test content");

        let encrypted_path = dir.path().join("test.vault");
        let key = Aes256Gcm::generate_key(OsRng);
        encrypt_file(input_path.to_str().unwrap(), &key).expect("encryption failed");

        _decrypt_file(encrypted_path.to_str().unwrap(), &key);

        let decrypted_path = Path::new("decrypted").join("test.txt");
        assert!(decrypted_path.exists(), "decrypted file not found");

        let decrypted_content = fs::read(&decrypted_path).expect("failed to read decrypted file");
        assert_eq!(original_content, decrypted_content.as_slice());

        fs::remove_file(&decrypted_path).ok();
    }
}
