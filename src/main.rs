#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/// jpdict/src/main.rs

mod ui;
mod db;
mod dictionary;

use eframe::{run_native, NativeOptions, CreationContext};
use ui::DictionaryApp;
use db::{init_db, populate_db};

fn main() {
    init_db().expect("Failed to initialize database");
    populate_db().expect("Failed to populate database");

    let native_options = NativeOptions::default();
    run_native(
        "Dictionary App",
        native_options,
        Box::new(|cc: &CreationContext| Box::new(DictionaryApp::new(cc))),
    ).expect("TODO: panic message");
}
