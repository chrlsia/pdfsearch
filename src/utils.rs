use glob::glob;

pub fn normalize_word(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

pub fn expand_dirs(patterns: &[String]) -> Vec<String> {
    let mut expanded = Vec::new();

    for pattern in patterns {
        match glob(pattern) {
            Ok(paths) => {
                let mut found = false;

                for path in paths.filter_map(Result::ok) {
                    if path.is_dir() {
                        expanded.push(path.to_string_lossy().to_string());
                        found = true;
                    }
                }

                if !found {
                    expanded.push(pattern.clone());
                }
            }
            Err(_) => expanded.push(pattern.clone()),
        }
    }

    expanded
}