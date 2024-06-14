/// jpdict/src/db/utils.rs

use crate::dictionary::DictionaryEntry;

pub fn calculate_score(entry: &DictionaryEntry, word: &str) -> i32 {
    let mut score = 0;
    let word_lower = word.to_lowercase();

    if entry.word.starts_with(word) { score += 150; }
    if entry.reading.starts_with(word) { score += 150; }
    if entry.pronunciation.starts_with(word) { score += 150; }

    if entry.word == word { score += 500; }
    if entry.reading == word { score += 500; }
    if entry.pronunciation == word { score += 5000; }

    if entry.word.contains(word) { score += 50; }
    if entry.reading.contains(word) { score += 50; }
    if entry.pronunciation.contains(word) { score += 50; }

    for meaning in &entry.translations {
        if meaning.to_lowercase() == word_lower { score += 30; }
        if meaning.to_lowercase().contains(&word_lower) { score += 10; }
    }

    score
}

pub fn replace_repeated_consonants(input: &str) -> String {
    let consonants = "bcdfghjklmnpqrstvwxyz";
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if consonants.contains(ch) {
            if let Some(&next_ch) = chars.peek() {
                if ch == next_ch {
                    result.push_str("tsu");
                    result.push(ch);
                    chars.next();
                    continue;
                }
            }
        }
        result.push(ch);
    }

    result
}
