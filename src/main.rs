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

            for (line_number, line) in content.lines().enumerate() {
                if line.contains("Rust") {
                    println!(
                        "Found in file: {:?} at line {} -> {}",
                        path, line_number + 1, line
                    );
                }
            }
        }
    }
}