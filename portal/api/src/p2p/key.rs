use base64::Engine;
use libp2p::identity;

pub fn serialize_keypair(keypair: identity::Keypair) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = keypair.to_protobuf_encoding()?;
    let data = base64::engine::general_purpose::STANDARD.encode(bytes);
    Ok(data)
}

pub fn deserialize_keypair(data: &str) -> Result<identity::Keypair, Box<dyn std::error::Error>> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(data)?;
    let keypair = identity::Keypair::from_protobuf_encoding(&bytes)?;
    Ok(keypair)
}

pub fn new_keypair() -> libp2p::identity::Keypair {
    libp2p::identity::Keypair::generate_ed25519()
}
