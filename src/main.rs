#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/// jpdict/src/main.rs

mod dictionary;
mod db;
mod ui;

use std::sync::Arc;
use eframe::{run_native, NativeOptions, CreationContext};
use eframe::egui::IconData;
use ui::DictionaryApp;
use db::{init_db, populate_db};

fn main() {
    init_db().expect("Failed to initialize database");
    populate_db().expect("Failed to populate database");

    let mut native_options = NativeOptions::default();
    let icon_data = include_bytes!("hso.png");
    let img = image::load_from_memory_with_format(icon_data, image::ImageFormat::Png).unwrap();
    let rgba_data = img.into_rgba8();
    let (w,h)=(rgba_data.width(),rgba_data.height());
    let raw_data: Vec<u8> = rgba_data.into_raw();
    native_options.viewport.icon=Some(Arc::<IconData>::new(IconData { rgba:  raw_data, width: w, height: h }));
    run_native(
        "JPDict",
        native_options,
        Box::new(|cc: &CreationContext| Box::new(DictionaryApp::new(cc))),
    ).expect("TODO: panic message");
}
