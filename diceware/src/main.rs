use std::fs::{ read_to_string };
use std::io::Error;
use std::path::Path;
use rand::Rng;

fn read_file(folder: &str, name: &str) -> Result<String, Error>{
    let path = Path::new(folder).join(name);
    let read_result = read_to_string(&path);
    return read_result;
}

fn main() {
    print!("\x1B[2J\x1B[1;1H\n");

    let content = read_file("", "english-words").expect("Error");
    let words_list: Vec<&str> = content.lines().filter(
        |w| !w.is_empty()
    ).collect();

    let mut i: u8;
    let mut final_password = String::new();
    let total_words_in_password: u8 = 6;
    let separator = "-"; // Pode ser gerado aleatoriamente tbm

    loop {
        final_password.clear();
        i = 0;

        while i < total_words_in_password {
            let random_number = rand::rng().random_range(0..=7775);
            if i != 0 {
                final_password.push_str(separator);
            }
            final_password.push_str(words_list[random_number]);
            i += 1;
        }

        // caso der o azar de pegar só palavras de 3 caracteres
        // faço rodar novamente o algoritmo pra obter uma senha
        // com maior entropia
        if final_password.len() >= 52 {
            break
        }
    }

    println!("{}", final_password);
}
