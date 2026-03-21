use sequoia_openpgp::{
    cert::Cert,
    parse::Parse,
    policy::StandardPolicy,
    serialize::stream::{Encryptor, LiteralWriter, Message},
};

use std::fs::File;
use std::io::{self, Read};

use anyhow::Result;

pub fn load_public_key(path: &str) -> Result<Cert> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(Cert::from_bytes(&buf)?)
}

pub fn encrypt_file_with_pgp(
    input_path: &str,
    output_path: &str,
    cert: &Cert,
) -> Result<(), Box<dyn std::error::Error>> {
    let policy = &StandardPolicy::new();

    let key = cert
        .keys()
        .with_policy(policy, None)
        .alive()
        .revoked(false)
        .for_transport_encryption()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No suitable encryption key found"))?;

    let mut input = File::open(input_path)?;
    let mut output = File::create(output_path)?;

    let message = Message::new(&mut output);
    let encryptor = Encryptor::for_recipients(message, vec![key]).build()?;

    let mut literal_writer = LiteralWriter::new(encryptor).build()?;
    io::copy(&mut input, &mut literal_writer)?;
    literal_writer.finalize()?;

    Ok(())
}
