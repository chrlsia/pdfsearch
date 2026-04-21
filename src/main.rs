use std::fs;

fn main() {
    let folder = "./test_folder";

    let entries = fs::read_dir(folder).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        let path_str = path.to_string_lossy();

        // For now: use .txt instead of .pdf
        if path_str.ends_with(".txt") {
            let content = fs::read_to_string(&path).unwrap();

            let query = "Rust safe"; // later this will come from user

            let words: Vec<&str> = query.split_whitespace().collect();

            for (line_number, line) in content.lines().enumerate() {
                let mut all_found = true;

                for word in &words {
                    if !line.contains(word) {
                        all_found = false;
                        break;
                    }
                }

                if all_found {
                    println!(
                        "Found in file: {:?} at line {} -> {}",
                        path,
                        line_number + 1,
                        line
                    );
                }
            }
        }
    }
}
