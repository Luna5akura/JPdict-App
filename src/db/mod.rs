/// jpdict/src/db/mod.rs

mod db_setup;
mod utils;

pub use db_setup::{init_db, populate_db};
pub use utils::{calculate_score, replace_repeated_consonants};
use rusqlite::{Connection, Result};
use crate::dictionary::DictionaryEntry;

pub async fn search_db(query: &str, page: usize, limit: usize) -> Result<Vec<DictionaryEntry>> {
    println!("Sending query: {}", query);
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }
    let query = query.to_lowercase();
    let query : &str = query.as_str();

    // let romaji_query = replace_repeated_consonants(&query);
    // println!("Sending romaji query: {}", romaji_query);

    let conn = Connection::open("dictionary.db")?;
    let mut stmt = conn.prepare(
        "SELECT word, reading, pos, inflection, freq, translations, sequence, tags, pronunciation
        FROM dictionary
        WHERE word LIKE ?1 OR reading LIKE ?1 OR translations LIKE ?1 OR pronunciation LIKE ?1",

    )?;

    let rows = stmt.query_map([format!("%{}%", query)], |row| {

    // let rows = stmt.query_map([format!("%{}%", query), format!("%{}%", romaji_query)], |row| {
        Ok(DictionaryEntry {
            word: row.get(0)?,
            reading: row.get(1)?,
            pos: row.get(2)?,
            inflection: row.get(3)?,
            freq: row.get(4)?,
            translations: serde_json::from_str(&row.get::<_, String>(5)?)
                .map_err(|e| rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_ERROR),
                    Some(e.to_string())
                ))?,
            sequence: row.get(6)?,
            tags: row.get(7)?,
            pronunciation: row.get(8)?,
        })
    })?;

    let mut entries: Vec<DictionaryEntry> = rows.filter_map(|res| res.ok()).collect();

    // Calculate scores
    let mut scored_entries: Vec<(i32, DictionaryEntry)> = entries.iter_mut()
        .map(|entry| {
            let score = calculate_score(entry, query);
            (score, entry.clone())
        })
        .filter(|(score, _)| *score > 0)
        .collect();

    println!("Found: {:?}", scored_entries);
    // Sort entries by score and frequency
    scored_entries.sort_by(|a, b| {
        b.0.cmp(&a.0).then_with(|| b.1.freq.cmp(&a.1.freq))
    });

    // Pagination
    let total_entries = scored_entries.len();
    let total_pages = (total_entries + limit - 1) / limit;
    let start = page * limit;
    let end = (start + limit).min(total_entries);
    let paginated_results: Vec<DictionaryEntry> = scored_entries[start..end]
        .iter()
        .map(|(_, entry)| entry.clone())
        .collect();
    // println!("Found {:?}", paginated_results);
    Ok(paginated_results)
}
