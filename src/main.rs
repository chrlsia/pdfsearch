use clap::Parser;
use walkdir::WalkDir;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::process::Command;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use colored::*;
#[derive(Parser)]
struct Args {
    /// Words to search (1–4 words)
    #[arg(short, long, num_args = 1..=4)]
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

// Store matches per file
struct MatchResult {
    file: String,
    matches: Vec<(usize, String)>,
}


fn highlight_line(line: &str, words: &[String]) -> String {
    let mut result = line.to_string();

    for word in words {
        let lower_word = word.to_lowercase();

        // Replace case-insensitively (simple approach)
        result = result
            .split_whitespace()
            .map(|token| {
                let normalized: String = token
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
                    .to_lowercase();

                if normalized == lower_word {
                    token.yellow().bold().to_string()
                } else {
                    token.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    }

    result
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

    // --- Parallel processing with result collection ---
    let results: Vec<MatchResult> = pdf_files
        .par_iter()
        .map(|entry| {
            let pb = pb.clone();
            let path = entry.path();

            let mut matches = Vec::new();

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
                    return None;
                }
            };

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
                    matches.push((line_number + 1, line.to_string()));
                }
            }

            pb.inc(1);

            if matches.is_empty() {
                None
            } else {
                Some(MatchResult {
                    file: path.to_string_lossy().to_string(),
                    matches,
                })
            }
        })
        .filter_map(|r| r)
        .collect();

    pb.finish_with_message("Done");

    // --- Sort results by file name ---
    let mut results = results;
    results.sort_by(|a, b| a.file.cmp(&b.file));

    // --- Print clean, ordered output ---
    for result in results {
        println!("\n📜File: {}", result.file.red().bold());

        for (line_number, line) in result.matches {
            let highlighted = highlight_line(&line, &words);
            println!("💰[{}] {}", line_number, highlighted);
        }
    }
}