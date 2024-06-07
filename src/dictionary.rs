/// jpdict/src/dictionary.rs

use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub reading: String,
    pub pos: String,
    pub inflection: Option<String>,
    pub freq: i32,
    pub translations: Vec<String>,
    pub sequence: i32,
    pub tags: Option<String>,
    pub pronunciation: String,
}