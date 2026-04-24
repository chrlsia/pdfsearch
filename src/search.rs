use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use std::process::Command;

use crate::cli::Args;
use crate::utils::{normalize_word, expand_dirs};
use crate::highlight::highlight_line;
use colored::*;
pub struct MatchResult {
    pub file: String,
    pub matches: Vec<(usize, String)>,
}

pub fn run_search(args: Args) {
    // Thread control
    if let Some(n) = args.threads {
        ThreadPoolBuilder::new()
            .num_threads(n)
            .build_global()
            .unwrap();
    }

    let words: Vec<String> = args.words.iter()
        .map(|w| normalize_word(w))
        .collect();

    let folders = expand_dirs(&args.dirs);

    println!("\nSearching in directories:\n {:?}\n", folders);

    let pdf_files: Vec<_> = folders.iter()
        .flat_map(|folder| {
            WalkDir::new(folder)
                .into_iter()
                .filter_map(|e| e.ok())
        })
        .filter(|e| e.path().is_file())
        .filter(|e| e.path().to_string_lossy().to_lowercase().ends_with(".pdf"))
        .collect();

    println!("Found {} PDF files.\n", pdf_files.len());

    let pb = Arc::new(ProgressBar::new(pdf_files.len() as u64));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap(),
    );

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

    let mut results = results;
    results.sort_by(|a, b| a.file.cmp(&b.file));

    for result in results {
        println!("\n📜File: {}", result.file.blue().bold());

        for (line_number, line) in result.matches {
            let highlighted = highlight_line(&line, &words);
            println!("💰[{}] {}", line_number, highlighted);
        }
    }
}