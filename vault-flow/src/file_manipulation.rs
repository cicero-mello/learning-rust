use std::fs::{ File, create_dir_all, read };
use std::io::prelude::Write;
use std::path::{ Path, PathBuf };

fn create_folder(folder_path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(folder_path);

    if path.exists() { return Ok(()); }

    let create_dir_result = create_dir_all(path);

    return match create_dir_result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

pub fn create_file(folder: &str, name: &str, content: &[u8]) {
    let create_folder_result = create_folder(folder);

    match create_folder_result {
        Err(e) => {
            eprintln!("Error to create folder '{}': {}", folder, e);
            return;
        },
        _ => {}
    }

    let mut file_path = PathBuf::from(folder);
    file_path.push(name);

    let mut file = File::create(&file_path).expect("Error to create file");

    file.write_all(content).expect("Error to write file");

    println!("\nFile Created! Checkout {:?}", file_path);
}

pub struct VaultFile {
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
    pub cipher_text: Vec<u8>
}

pub fn read_file(folder: &str, name: &str) -> VaultFile {
    let path = Path::new(folder).join(name);

    let file_bytes = read(path).expect("Erro ao ler o arquivo");

    if file_bytes.len() < 28 {
        panic!("Arquivo inválido: tamanho insuficiente para conter salt e nonce");
    }

    // Extrai os dados
    let salt = file_bytes[0..16].to_vec();
    let nonce = file_bytes[16..28].to_vec();
    let cipher_text = file_bytes[28..].to_vec();

    println!("Salt:\n{:?}", salt);
    println!("Nonce:\n{:?}", nonce);
    println!("Cipher Text:\n{:?}", cipher_text);

    return VaultFile { salt, nonce, cipher_text };
}
