use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("./test_folder") {
        let entry = entry.unwrap();
        let path = entry.path();

        let path_str = path.to_string_lossy();

        if path_str.ends_with(".pdf") {
            println!("PDF: {:?}", path);
        }
    }
}
