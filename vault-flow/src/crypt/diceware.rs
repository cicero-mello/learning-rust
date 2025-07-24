use std::fs::{ read_to_string };
use std::io::Error;
use std::path::Path;
use rand::Rng;

fn read_words_file(name: &str) -> Result<String, Error>{
    let path = Path::new(".dice-words").join(name);
    let read_result = read_to_string(&path);
    return read_result;
}

const SEPARATORS: [char; 7] = ['&', '+', '/', '=', '*', '%', '['];

fn get_random_separator() -> char {
    let index = rand::rng().random_range(0..SEPARATORS.len());
    return SEPARATORS[index];
}

pub fn generate(words_quantity: u8) -> String {

    let content = read_words_file("english").expect("Error");
    let words_list: Vec<&str> = content.lines().filter(
        |w| !w.is_empty()
    ).collect();

    let mut i: u8;
    let mut final_password = String::new();
    let separator = get_random_separator();

    loop {
        final_password.clear();
        i = 0;

        while i < words_quantity {
            let random_number = rand::rng().random_range(0..=7775);
            if i != 0 {
                final_password.push(separator);
            }
            final_password.push_str(words_list[random_number]);
            i += 1;
        }

        if final_password.len() >= 52 {
            break
        }
    }

    return final_password;
}
