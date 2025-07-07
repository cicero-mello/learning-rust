/*
    Cargo é a ferramenta de build + gerenciador de pacotes
    oficial do RUST.

    A parte de gerenciamento de pacotes fica no arquivo .toml,
    que funciona de forma similar ao package.json no NodeJS.

    Ao fazer o build, agora são criados mais arquivos:
    - Cargo.lock: contém as versões das dependências do ultimo build.
    - /target: pasta com varias coisas geradas, das quais to com
    preguiça de pesquisar uma por uma agora. O mais relevante
    nesse momento, é que em ./target/debug temos o arquivo binário
    que pode ser executado (nesse projeto, é o "hello-cargo")
*/

fn main() {
    println!("Hello, world! WoW!");
}

// Para rodar: "cargo run"
/*
    Isso irá realizar a compilação, e depois,
    executar o resultado.
*/

// Para checar o código (sem build ou execução): "cargo check"

// Para compilação final (release): "cargo build --release"
/*
    fazendo o build com --release, os arquivos gerados vao para
    ./target/release em vez de ./target/debug
*/
