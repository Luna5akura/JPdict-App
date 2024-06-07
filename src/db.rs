/// jpdict/src/db.rs

use rusqlite::{Connection, Result, params};
use crate::dictionary::DictionaryEntry;
use std::fs;

pub fn init_db() -> Result<()> {
    let conn = Connection::open("dictionary.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS dictionary (
            id INTEGER PRIMARY KEY,
            word TEXT NOT NULL,
            reading TEXT NOT NULL,
            pos TEXT NOT NULL,
            inflection TEXT NOT NULL,
            freq INTEGER NOT NULL,
            translations TEXT NOT NULL,
            sequence INTEGER NOT NULL,
            tags TEXT NOT NULL,
            pronunciation TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}


pub fn populate_db() -> Result<()> {
    println!("Current working directory: {:?}", std::env::current_dir());

    let mut conn = Connection::open("dictionary.db")?;
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM dictionary", [], |row| row.get(0))?;
    if count > 0 {
        println!("Database already populated. Skipping data insertion.");
        return Ok(());
    }
    let tx = conn.transaction()?;

    for i in 1..=2 {
        let filename = format!("../assets/test_term_bank_{}.json", i);
        let data = fs::read_to_string(&filename)
            .map_err(|e| rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_ERROR),
                Some(e.to_string())
            ))?;

        let entries: Vec<DictionaryEntry> = serde_json::from_str(&data)
            .map_err(|e| rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_ERROR),
                Some(e.to_string())
            ))?;

        for entry in entries {
            if !entry.word.is_empty() {
                tx.execute(
                    "INSERT OR IGNORE INTO dictionary (word, reading, pos, inflection, freq, translations, sequence, tags, pronunciation)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    params![
                        entry.word,
                        entry.reading,
                        entry.pos,
                        entry.inflection.unwrap_or_default(),
                        entry.freq as i32,
                        serde_json::to_string(&entry.translations)
                            .map_err(|e| rusqlite::Error::SqliteFailure(
                                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_ERROR),
                                Some(e.to_string())
                            ))?,
                        entry.sequence as i32,
                        entry.tags.unwrap_or_default(),
                        entry.pronunciation,
                    ],
                )?;
            }
        }
    }

    tx.commit()?;
    Ok(())
}

fn calculate_score(entry: &DictionaryEntry, word: &str) -> i32 {
    let mut score = 0;
    let word_lower = word.to_lowercase();

    if entry.word.starts_with(word) { score += 15; }
    if entry.reading.starts_with(word) { score += 15; }
    if entry.pos.starts_with(word) { score += 15; } // Assuming pos is used for romaji
    if entry.pronunciation.starts_with(word) { score += 15; }

    if entry.word == word { score += 10; }
    if entry.reading == word { score += 10; }
    if entry.pos == word { score += 10; } // Assuming pos is used for romaji
    if entry.pronunciation == word { score += 10; }

    if entry.word.contains(word) { score += 5; }
    if entry.reading.contains(word) { score += 5; }
    if entry.pos.contains(word) { score += 5; } // Assuming pos is used for romaji
    if entry.pronunciation.contains(word) { score += 5; }

    for meaning in &entry.translations {
        if meaning.to_lowercase() == word_lower { score += 3; }
        if meaning.to_lowercase().contains(&word_lower) { score += 1; }
    }

    score
}

pub fn search_db(query: &str, page: usize, limit: usize) -> Result<Vec<DictionaryEntry>> {
    println!("Sending query: {}", query);
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    let conn = Connection::open("dictionary.db")?;
    let mut stmt = conn.prepare(
        "SELECT word, reading, pos, inflection, freq, translations, sequence, tags, pronunciation
        FROM dictionary
        WHERE word LIKE ?1 OR reading LIKE ?1 OR translations LIKE ?1 OR pronunciation LIKE ?1",
    )?;

    let rows = stmt.query_map([format!("%{}%", query)], |row| {
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

    Ok(paginated_results)
}
