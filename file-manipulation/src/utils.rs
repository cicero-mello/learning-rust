use std::fs::{ File, create_dir_all, read_to_string };
use std::io::prelude::Write; // para usar write_all
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

// pub funciona tipo como um "export"
pub fn create_file(folder: &str, name: &str, content: &str) {
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

    file.write_all(content.as_bytes()).expect("Error to write file");

    println!("\nFile Created! Checkout {:?}", file_path);
}

pub fn read_file(folder: &str, name: &str) {
    let path = Path::new(folder).join(name);

    let read_result = read_to_string(&path);

    match read_result {
        Ok(content) => println!("\nContent:\n{}", content),
        Err(e) => eprint!("\nFail to read: {}", e)
    };
}
