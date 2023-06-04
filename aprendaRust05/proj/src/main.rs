use std::io;

fn main() {
    let mut s = String::new();
    println!("Digite um texto");

    io::stdin()
        .read_line(&mut s) // Result
        .expect("Error reading console");

    println!("Vocề digitou {s}")
}
