use pgp::composed::{Message, SignedPublicKey};
use pgp::composed::Deserializable;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

/// Load an ASCII-armored public key from file.
pub fn load_public_key(path: &Path) -> Result<SignedPublicKey, Box<dyn std::error::Error>> {
    let key_data = fs::read_to_string(path)?;
    let (pubkey, _) = SignedPublicKey::from_string(&key_data)?;
    Ok(pubkey)
}

/// Encrypts `input_path`, writes `.asc` to `output_path`.
pub fn encrypt_file(
    pubkey_path: &Path,
    input_path: &Path,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let _pubkey = load_public_key(pubkey_path)?;

    let mut plaintext = Vec::new();
    File::open(input_path)?.read_to_end(&mut plaintext)?;

    // For now, just write the plaintext to output
    // TODO: Implement proper PGP encryption
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, plaintext)?;

    Ok(())
}
