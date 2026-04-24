use colored::*;
use crate::utils::normalize_word;

pub fn highlight_line(line: &str, words: &[String]) -> String {
    line.split_whitespace()
        .map(|token| {
            let start = token
                .char_indices()
                .find(|(_, c)| c.is_alphanumeric())
                .map(|(i, _)| i)
                .unwrap_or(0);

            let end = token
                .char_indices()
                .rev()
                .find(|(_, c)| c.is_alphanumeric())
                .map(|(i, c)| i + c.len_utf8())
                .unwrap_or(token.len());

            let (prefix, rest) = token.split_at(start);
            let (core, suffix) = rest.split_at(end - start);

            let normalized = normalize_word(core);

            if words.contains(&normalized) {
                format!("{}{}{}", prefix, core.yellow().bold(), suffix)
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}