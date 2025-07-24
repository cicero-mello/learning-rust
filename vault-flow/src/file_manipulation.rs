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
}

pub struct VaultFile {
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
    pub cipher_text: Vec<u8>
}

pub fn read_vault_file(folder: &str, name: &str) -> VaultFile {
    let path = Path::new(folder).join(name);

    let file_bytes = read(path).expect("Error");

    if file_bytes.len() < 28 {
        panic!("Invalid File Size");
    }

    let salt = file_bytes[0..16].to_vec();
    let nonce = file_bytes[16..28].to_vec();
    let cipher_text = file_bytes[28..].to_vec();

    return VaultFile { salt, nonce, cipher_text };
}

pub fn list_files(folder: &str) -> Result<Vec<String>, std::io::Error> {
    let path = Path::new(folder);

    if !path.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Pasta não encontrada"));
    }

    let mut file_names = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                file_names.push(name.to_string());
            }
        }
    }

    Ok(file_names)
}
