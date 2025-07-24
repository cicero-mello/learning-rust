use aes_gcm::{
    Aes256Gcm,
    aead::Nonce
};

use rand::RngCore;
use std::time::{ SystemTime, UNIX_EPOCH };

pub fn generate_nonce() -> Nonce<Aes256Gcm> {

    let duration = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error");
    let timestamp_ms = duration.as_millis() as u64;

    let timestamp_bytes = &timestamp_ms.to_be_bytes()[2..8];

    let mut random_bytes = [0u8; 6];
    rand::rng().fill_bytes(&mut random_bytes);

    let mut nonce_bytes = [0u8; 12];
    nonce_bytes[..6].copy_from_slice(timestamp_bytes);
    nonce_bytes[6..].copy_from_slice(&random_bytes);

    Nonce::<Aes256Gcm>::from_slice(&nonce_bytes).to_owned()
}
