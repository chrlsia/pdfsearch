use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Words to search (1–3 words)
    #[arg(short, long, num_args = 1..=3)]
    words: Vec<String>,

    /// Directory containing PDFs
    #[arg(short, long)]
    dir: String,
}

fn main() {
    let args = Args::parse();

    println!("Words: {:?}", args.words);
    println!("Directory: {}", args.dir);
}
/*
cargo r -q -- -w siannas chris -d ./chris

Words: ["siannas", "chris"]
Directory: ./chris
*/