use aes_gcm::{
    Key,
    Aes256Gcm,
    aead::{ KeyInit, OsRng }
};

pub fn generate_key() -> Key<Aes256Gcm>{
    let key = Aes256Gcm::generate_key(OsRng);
    return key
}
