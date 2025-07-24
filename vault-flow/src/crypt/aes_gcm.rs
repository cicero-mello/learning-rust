use crate::crypt::nonce::generate_nonce;

use aes_gcm::{
    aead::{Aead, KeyInit, Nonce },
    Aes256Gcm, Key
};

pub struct EncryptedData {
    pub cipher_text: Vec<u8>,
    pub nonce: Vec<u8>
}

pub fn encrypt_content(
    content: &str,
    key: Key<Aes256Gcm>,
    nonce_option: Option<Nonce<Aes256Gcm>>
) -> EncryptedData {

    let nonce = match nonce_option {
        Some(value) => value,
        None => generate_nonce()
    };

    let cipher = Aes256Gcm::new(&key);
    let cipher_text = cipher.encrypt(&nonce, content.as_bytes()).expect("Error");

    return EncryptedData {
        cipher_text,
        nonce: nonce.to_vec()
    };
}

pub fn decrypt_content(
    cipher_text: Vec<u8>,
    key: Key<Aes256Gcm>,
    nonce: &Nonce<Aes256Gcm>
) -> String {
    let cipher = Aes256Gcm::new(&key);
    let plaintext = cipher.decrypt(
        &nonce,
        cipher_text.as_ref()
    ).expect("Error aq");

    return String::from_utf8_lossy(&plaintext).to_string();
}
