use pdf_extract::extract_text;

fn main() {
    let text = extract_text("./test_folder/example.pdf").unwrap();

    println!("{}", text);
}
