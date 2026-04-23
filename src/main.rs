use clap::Parser;
use walkdir::WalkDir;
use rayon::prelude::*;
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

    // --- Collect all PDF files first ---
    let pdf_files: Vec<_> = WalkDir::new(&folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            e.path()
                .to_string_lossy()
                .to_lowercase()
                .ends_with(".pdf")
        })
        .collect();

    println!("Found {} PDF files.\n", pdf_files.len());

    // --- Process files in parallel ---
    pdf_files.par_iter().for_each(|entry| {
        let path = entry.path();

        println!("🔎 Scanning file: {:?}...", path);

        // Run pdftotext
        let output = Command::new("pdftotext")
            .arg(path)
            .arg("-")
            .output();

        let content = match output {
            Ok(out) if out.status.success() => {
                String::from_utf8_lossy(&out.stdout).to_string()
            }
            Ok(_) => {
                println!("pdftotext failed on: {:?}", path);
                return;
            }
            Err(e) => {
                println!("Error running pdftotext on {:?}: {}", path, e);
                return;
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
                println!("💰 {}", line);
            }
        }
    });
}