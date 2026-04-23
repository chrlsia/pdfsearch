use clap::Parser;
use walkdir::WalkDir;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::process::Command;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;

#[derive(Parser)]
struct Args {
    /// Words to search (1–3 words)
    #[arg(short, long, num_args = 1..=3)]
    words: Vec<String>,

    /// Directories containing PDFs (supports multiple)
    #[arg(short, long, num_args = 1..)]
    dirs: Vec<String>,

    /// Number of threads (optional)
    #[arg(short = 't', long)]
    threads: Option<usize>,
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

    // --- Configure Rayon threads ---
    if let Some(n) = args.threads {
        ThreadPoolBuilder::new()
            .num_threads(n)
            .build_global()
            .unwrap();
    }

    // Normalize search words
    let words: Vec<String> = args
        .words
        .iter()
        .map(|w| normalize_word(w))
        .collect();

    let folders = args.dirs;

    // --- Collect all PDF files ---
    let pdf_files: Vec<_> = folders
        .iter()
        .flat_map(|folder| {
            WalkDir::new(folder)
                .into_iter()
                .filter_map(|e| e.ok())
        })
        .filter(|e| e.path().is_file())
        .filter(|e| {
            e.path()
                .to_string_lossy()
                .to_lowercase()
                .ends_with(".pdf")
        })
        .collect();

    println!("Found {} PDF files.\n", pdf_files.len());

    // --- Progress bar ---
    let pb = Arc::new(ProgressBar::new(pdf_files.len() as u64));

    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {pos}/{len} files")
            .unwrap(),
    );

    // --- Parallel processing ---
    pdf_files.par_iter().for_each(|entry| {
        let pb = pb.clone();

        let path = entry.path();

        // Run pdftotext
        let output = Command::new("pdftotext")
            .arg(path)
            .arg("-")
            .output();

        let content = match output {
            Ok(out) if out.status.success() => {
                String::from_utf8_lossy(&out.stdout).to_string()
            }
            _ => {
                pb.inc(1);
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
                println!("> {}", line);
            }
        }

        pb.inc(1);
    });

    pb.finish_with_message("Done");
}