mod key;
mod nonce;
use crate::{
    key::generate_key,
    nonce::generate_nonce
};

use aes_gcm::{
    aead::{Aead, KeyInit },
    Aes256Gcm
};


fn main() {
    print!("\x1B[2J\x1B[1;1H\n");

    let key = generate_key();
    let nonce = generate_nonce();
    let text = "Heyy, I'm a text that will be encrypted! ºoº 😯";

    println!("{}\n", "─".repeat(128));
    println!("Text to be encrypted:\n{}", text);
    println!("\nIn Bytes: \n{:?}\n", text.as_bytes());


    let cipher = Aes256Gcm::new(&key);
    let cipher_text = cipher.encrypt(&nonce, text.as_bytes().as_ref()).expect("Error");

    println!("{}\n", "─".repeat(128));
    println!("Encrypted text in Bytes:\n{:?}", &cipher_text);
    println!(
        "\nTrying to convert this Bytes in String: \n{}\n",
        String::from_utf8_lossy(&cipher_text)
    );


    let plaintext = cipher.decrypt(&nonce, cipher_text.as_ref()).expect("Error");

    println!("{}\n", "─".repeat(128));
    println!("Decrypted text in Bytes:\n{:?}\n", plaintext);
    println!(
        "\nDecrypted text converted in String:\n{}",
        String::from_utf8_lossy(&plaintext)
    );
}
