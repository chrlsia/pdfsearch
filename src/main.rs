use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("./test_folder") {
        let entry = entry.unwrap();
        println!("{:?}", entry.path());
    }
}