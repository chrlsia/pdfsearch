use std::fs;

fn main() {
    let content = fs::read_to_string("./test_folder/example.txt").unwrap();

    for line in content.lines() {
        if line.contains("Rust") {
            println!("Found: {}", line);
        }
    }
}