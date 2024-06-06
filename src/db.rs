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


pub fn search_db(query: &str) -> Result<Vec<DictionaryEntry>> {
    println!("Sending query: {}", query);

    let conn = Connection::open("dictionary.db")?;
    let mut stmt = conn.prepare(
        "SELECT word, reading, pos, inflection, freq, translations, sequence, tags, pronunciation
        FROM dictionary
        WHERE word LIKE ?1 OR reading LIKE ?1",
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

    let entries: Result<Vec<_>> = rows.collect();
    if let Ok(ref entries) = entries {
        println!("Received {} results", entries.len());
    } else if let Err(ref err) = entries {
        println!("Error occurred while searching: {:?}", err);
    }
    entries
}
