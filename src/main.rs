use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run <word1> [word2] [word3] <folder>");
        return;
    }

    let folder = args.last().unwrap();

    let words: Vec<String> = args[1..args.len() - 1]
        .iter()
        .map(|w| w.to_lowercase())
        .collect();

    println!("Folder: {}", folder);
    println!("Words: {:?}", words);
}