use aes_gcm::{
    Aes256Gcm,
    aead::{ AeadCore, OsRng, Nonce }
};

pub fn generate_nonce () -> Nonce<Aes256Gcm> {
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    return nonce;
}
