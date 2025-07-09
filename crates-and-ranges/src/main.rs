/*
    "Crate" é um código fonte em rust.
    Esse projeto, por exemplo, é um "binary crate",
    que se trata de um executável.
    Porém, existem outro tipo de "Crate",
    o "Library Crate", que são códigos sem um executável final,
    feitos para serem usados em outras "Crates" (binary ou lib).
*/

/*
    No arquivo Cargo.toml, foi adicionado a dependência
    "rand", que é um conjunto de Crates sobre aleatoriedade.

    No exemplo abaixo, está como usar a feature de geração
    de numero aleatória (random number generation, aka RNG)
*/

use rand::Rng;

fn main() {
    print!("\x1B[2J\x1B[1;1H\n");
    let number_between_1_and_10 = rand::rng().random_range(1..=10);
    println!("\n|| {} ||\n", number_between_1_and_10);

    // -------

    println!("Showing Numbers From 0 to 4:");
    for i in 0..5 {
        println!("i: {}", i);
    }

    // -------

    let arr = [10, 20, 30, 40, 50];
    println!("\nShowing Sliced Array From 20 to 40:");
    println!("{:?}\n", &arr[1..4]);
}

/*
    O "1..=10" é uma "Range Expression", abaixo tem uma tabelinha
    pra entender seus possíveis valores.
    Ela pode ser usada em diversos exemplos.

    a..b	|   de "a" até "b", sem incluir "b"
    a..=b	|   de "a" até "b", incluindo "b"
    ..b	    |   de "início implícito" até antes de "b"
    ..=b	|   de "início implícito" até "b"
    a..	    |   de "a" até o fim
    ..	    |   do início até o fim
*/
