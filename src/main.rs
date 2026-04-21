use std::fs;

fn main() {
    let path = "./test_folder";

    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let file_path = entry.path();

        println!("{:?}", file_path);
    }
}