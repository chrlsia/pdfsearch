use clap::Parser;
use walkdir::WalkDir;
use pdf_extract::extract_text;

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
    // --- Parse CLI arguments ---
    let args = Args::parse();

    // Normalize words (lowercase)
    let words: Vec<String> = args
        .words
        .iter()
        .map(|w| w.to_lowercase())
        .collect();

    let folder = args.dir;

    // --- Recursive directory traversal ---
    for entry in WalkDir::new(&folder) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, // skip unreadable entries
        };

        let path = entry.path();

        // Only process files ending with .pdf
        if path.is_file() {
            let path_str = path.to_string_lossy();

            if path_str.ends_with(".pdf") {
                println!("Scanning file: {:?}...", path);

                // Extract text from PDF
                let content = match extract_text(&path) {
                    Ok(text) => text,
                    Err(_) => {
                        println!("Could not read PDF: {:?}", path);
                        continue;
                    }
                };

                // --- Search lines ---
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

/*
 cargo run -- -w rust safe -d ./test_folder
   Compiling seach_pdf v0.1.0 (/home/cs/Documents/rust_projects/chatGPT/seach_pdf)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running `target/debug/seach_pdf -w rust safe -d ./test_folder`
Scanning file: "./test_folder/exa2.pdf"...

Found in file: "./test_folder/exa2.pdf" at line 4
> Rust is memory safe
Scanning file: "./test_folder/Lesson+11+-+Time+Order+Words.pdf"...
Scanning file: "./test_folder/exa1.pdf"...

Found in file: "./test_folder/exa1.pdf" at line 4
> Rust is memory safe
Scanning file: "./test_folder/example.pdf"...

Found in file: "./test_folder/example.pdf" at line 4
> Rust is memory safe
*/