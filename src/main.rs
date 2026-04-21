use std::fs;
use std::io;
use pdf_extract::extract_text;

fn main() {
    // --- 1. Read user input ---
    let mut query = String::new();

    println!("Enter search words (1–3 words):");

    io::stdin()
        .read_line(&mut query)
        .unwrap();

    let query = query.trim().to_string();

    // --- 2. Prepare words ---
    let words: Vec<String> = query
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();

    let folder = "./test_folder";

    // --- 3. Read folder ---
    let entries = fs::read_dir(folder).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_string_lossy();

        if path_str.ends_with(".pdf") {
            println!("Scanning file: {:?}...", path);

            let content = extract_text(&path).unwrap();

            // --- 4. Search lines ---
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
                    println!(
                        "\nFound in file: {:?} at line {}",
                        path, line_number + 1
                    );
                    println!("> {}", line);
                }
            }
        }
    }
}