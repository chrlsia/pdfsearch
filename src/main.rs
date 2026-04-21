use std::env;
use std::fs;
use pdf_extract::extract_text;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run <word1> [word2] [word3] <folder>");
        return;
    }

    let folder = args.last().unwrap();

    let words: Vec<String> = args[1..args.len() - 1]
        .iter()
        .map(|w| w.to_lowercase())
        .collect();

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