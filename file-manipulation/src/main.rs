mod utils;
use std::io::stdin;
use crate::utils::read_file;

fn main() {
    print!("\x1B[2J\x1B[1;1H\n");

    println!("Let's create a file! 😎📄\n");

    println!("Type your text:");
    let mut text = String::new();
    stdin().read_line(&mut text).expect("Error");

    println!("\nType your file name:");
    let mut file_name = String::new();
    stdin().read_line(&mut file_name).expect("Error");

    println!("\nType the folder to put the file in:");
    println!("(ex: outputs/secrets)");
    let mut folder = String::new();
    stdin().read_line(&mut folder).expect("Error");

    utils::create_file(
        folder.as_str().trim(),
        file_name.as_str().trim(),
        text.as_str()
    );

    println!("Want to read them? (y/n)");
    let mut answer = String::new();

    loop {
        stdin().read_line(&mut answer).expect("Error");
        let answer_str = answer.as_str().trim();

        if answer_str == "y" {
            read_file(folder.as_str().trim(), file_name.as_str().trim());
            break;
        }
        else if answer_str == "n" {
            break;
        }
        println!("Invalid answer, try again (with 'y' our 'n'):");
    }
}
