use std::fs;
use pdf_extract::extract_text;

fn main() {
    let folder = "./test_folder";
    let query = "rust safe";

    let words: Vec<String> = query
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();

    let entries = fs::read_dir(folder).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_string_lossy();

        if path_str.ends_with(".pdf") {
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
                    println!(
                        "Found in file: {:?} at line {} -> {}",
                        path, line_number + 1, line
                    );
                }
            }
        }
    }
}