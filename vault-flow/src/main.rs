use crate::vault::create::create_vault;

mod crypto;
mod vault;
mod file_manipulation;

fn main() {

    create_vault("teste", "pass123");
}
