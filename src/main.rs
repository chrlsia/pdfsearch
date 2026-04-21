use clap::Parser;
use pdf_extract::extract_text;
use std::fs;

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

    let words: Vec<String> = args.words.iter().map(|w| w.to_lowercase()).collect();

    let folder = args.dir;

    if words.is_empty() || words.len() > 3 {
        println!("Please provide 1 to 3 words.");
        return;
    }

    let entries = fs::read_dir(folder).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_string_lossy();

        if path_str.ends_with(".pdf") {
            println!("Scanning file: {:?}...", path);

            let content = extract_text(&path).unwrap();

            for (line_number, line) in content.lines().enumerate() {
                let line_lower = line.to_lowercase();

                let mut all_found = true;

                for word in &words {
                    if !line_lower.contains(word) {
                        all_found = false;
                        break;
                    }
                }

                if all_found {
                    println!("\nFound in file: {:?} at line {}", path, line_number + 1);
                    println!("> {}", line);
                }
            }
        }
    }
}
