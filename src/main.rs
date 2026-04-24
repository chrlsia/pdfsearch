use clap::Parser;
use colored::*;
use glob::glob;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::process::Command;
use std::sync::Arc;
use walkdir::WalkDir;
use colored;

// --- CLI ---
#[derive(Parser)]
struct Args {
    #[arg(short, long, num_args = 1..=3)]
    words: Vec<String>,

    #[arg(short, long, num_args = 1..)]
    dirs: Vec<String>,

    #[arg(short = 't', long)]
    threads: Option<usize>,
}

// --- Normalize words ---
fn normalize_word(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

// --- Highlight only exact word inside token ---
fn highlight_line(line: &str, words: &[String]) -> String {
    line.split_whitespace()
        .map(|token| {
            let start = token.chars().position(|c| c.is_alphanumeric()).unwrap_or(0);

            let end = token
                .char_indices()
                .rev()
                .find(|(_, c)| c.is_alphanumeric())
                .map(|(i, c)| i + c.len_utf8())
                .unwrap_or(token.len());

            let (prefix, rest) = token.split_at(start);
            let (core, suffix) = rest.split_at(end - start);

            let normalized = normalize_word(core);

            if words.contains(&normalized) {
                format!("{}{}{}", prefix, core.yellow().bold(), suffix)
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// --- Result struct ---
struct MatchResult {
    file: String,
    matches: Vec<(usize, String)>,
}

fn main() {
    let args = Args::parse();

    // --- Thread control ---
    if let Some(n) = args.threads {
        ThreadPoolBuilder::new()
            .num_threads(n)
            .build_global()
            .unwrap();
    }

    let words: Vec<String> = args.words.iter().map(|w| normalize_word(w)).collect();

    // --- Expand directories (cross-platform glob support) ---
    let mut expanded_dirs = Vec::new();

    for pattern in &args.dirs {
        match glob(pattern) {
            Ok(paths) => {
                let mut found = false;

                for path in paths.filter_map(Result::ok) {
                    if path.is_dir() {
                        expanded_dirs.push(path.to_string_lossy().to_string());
                        found = true;
                    }
                }

                if !found {
                    expanded_dirs.push(pattern.clone());
                }
            }
            Err(_) => expanded_dirs.push(pattern.clone()),
        }
    }

    println!("Searching in directories: {:?}", expanded_dirs);

    // --- Collect PDFs ---
    let pdf_files: Vec<_> = expanded_dirs
        .iter()
        .flat_map(|folder| WalkDir::new(folder).into_iter().filter_map(|e| e.ok()))
        .filter(|e| e.path().is_file())
        .filter(|e| e.path().to_string_lossy().to_lowercase().ends_with(".pdf"))
        .collect();

    println!("Found {} PDF files.\n", pdf_files.len());

    // --- Progress bar ---
    let pb = Arc::new(ProgressBar::new(pdf_files.len() as u64));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap(),
    );

    // --- Parallel processing ---
    let results: Vec<MatchResult> = pdf_files
        .par_iter()
        .map(|entry| {
            let pb = pb.clone();
            let path = entry.path();

            let mut matches = Vec::new();

            let output = Command::new("pdftotext").arg(path).arg("-").output();

            let content = match output {
                Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).to_string(),
                _ => {
                    pb.inc(1);
                    return None;
                }
            };

            for (line_number, line) in content.lines().enumerate() {
                let line_words: Vec<String> =
                    line.split_whitespace().map(|w| normalize_word(w)).collect();

                if words.iter().all(|w| line_words.contains(w)) {
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

    // --- Sort results ---
    let mut results = results;
    results.sort_by(|a, b| a.file.cmp(&b.file));

    // --- Print output ---
    for result in results {
        println!("\n📜 File: {}", result.file.cyan());

        for (line_number, line) in result.matches {
            let highlighted = highlight_line(&line, &words);
            println!("💰 [{}] {}", line_number, highlighted);
        }
    }
}
