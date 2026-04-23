use clap::Parser;
use walkdir::WalkDir;
use std::process::Command;

#[derive(Parser)]
struct Args {
    /// Words to search (1–3 words)
    #[arg(short, long, num_args = 1..=3)]
    words: Vec<String>,

    /// Directory containing PDFs
    #[arg(short, long)]
    dir: String,
}

// Normalize words: remove punctuation + lowercase
fn normalize_word(word: &str) -> String {
    word
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

fn main() {
    // --- Parse CLI arguments ---
    let args = Args::parse();

    // Normalize search words
    let words: Vec<String> = args
        .words
        .iter()
        .map(|w| normalize_word(w))
        .collect();

    let folder = args.dir;

    // --- Recursive directory traversal ---
    for entry in WalkDir::new(&folder) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.is_file() {
            let path_str = path.to_string_lossy();

            // Case-insensitive .pdf check
            if path_str.to_lowercase().ends_with(".pdf") {
                println!("Scanning file: {:?}...", path);

                // --- Call pdftotext ---
                let output = Command::new("pdftotext")
                    .arg(&path)
                    .arg("-")
                    .output();

                let content = match output {
                    Ok(out) if out.status.success() => {
                        String::from_utf8_lossy(&out.stdout).to_string()
                    }
                    Ok(_) => {
                        println!("pdftotext failed on: {:?}", path);
                        continue;
                    }
                    Err(e) => {
                        println!("Error running pdftotext on {:?}: {}", path, e);
                        continue;
                    }
                };

                // --- Search lines ---
                for (line_number, line) in content.lines().enumerate() {
                    let line_words: Vec<String> = line
                        .split_whitespace()
                        .map(|w| normalize_word(w))
                        .collect();

                    let mut all_found = true;

                    for word in &words {
                        if !line_words.contains(word) {
                            all_found = false;
                            break;
                        }
                    }

                    if all_found {
                        println!(
                            "\nFound in file: {:?} at line {}",
                            path,
                            line_number + 1
                        );
                        println!("> {}", line);
                    }
                }
            }
        }
    }
}