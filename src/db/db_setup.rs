/// jpdict/src/db/db_setup.rs

use rusqlite::{Connection, Result, params};
use crate::dictionary::DictionaryEntry;
use serde_json::Value;
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
    println!("Populating database...");
    for i in 1..=34 {
        println!("Processing file {}...", i);

        let filename = format!("assets/new_term_bank_{}.json", i);
        // TODO:Directory change
        let data = fs::read_to_string(&filename).map_err(|e| {
            rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_ERROR), Some(e.to_string()))
        })?;

        let json_value: Value = match serde_json::from_str(&data) {
            Ok(value) => value,
            Err(e) => {
                println!("Error parsing JSON from file {}: {}", filename, e);
                continue; // Skip the file if JSON parsing fails
            }
        };

        if let Some(entries) = json_value.as_array() {
            for entry_value in entries {
                let entry: DictionaryEntry = match serde_json::from_value(entry_value.clone()) {
                    //TODO: map
                    Ok(entry) => entry,
                    Err(e) => {
                        println!("Error parsing entry in file {}: {}", filename, e);
                        continue; // Skip this entry if parsing fails
                    }
                };

                if entry.word.is_empty() {
                    continue; // Skip entries with empty words
                }

                let translations = match serde_json::to_string(&entry.translations) {
                    Ok(trans) => trans,
                    Err(e) => {
                        println!("Error serializing translations for word {}: {}", entry.word, e);
                        continue; // Skip this entry if serialization fails
                    }
                };

                if let Err(e) = tx.execute(
                    "INSERT OR IGNORE INTO dictionary (word, reading, pos, inflection, freq, translations, sequence, tags, pronunciation)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    params![
                        entry.word,
                        entry.reading,
                        entry.pos,
                        entry.inflection.unwrap_or_default(),
                        entry.freq as i32,
                        translations,
                        entry.sequence as i32,
                        entry.tags.unwrap_or_default(),
                        entry.pronunciation,
                    ],
                ) {
                    println!("Error inserting entry for word {}: {}", entry.word, e);
                    // Skip this entry if insertion fails
                }
            }
        } else {
            println!("Error: expected an array of entries in file {}", filename);
        }
    }

    tx.commit()?;
    Ok(())
}
