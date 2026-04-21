use std::fs;

fn main() {
    let path = "./test_folder";

    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let file_path = entry.path();

        // Convert path to string (so we can check it)
        let path_str = file_path.to_string_lossy();

        if path_str.ends_with(".pdf") {
            println!("{:?}", file_path);
        }
    }
}