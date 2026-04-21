use std::io;

fn main() {
    let mut query = String::new();

    println!("Enter search words (1–3 words):");

    io::stdin()
        .read_line(&mut query)
        .unwrap();

    let query = query.trim().to_string();

    println!("You searched for: {}", query);
}