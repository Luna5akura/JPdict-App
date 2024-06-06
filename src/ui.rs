#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/// jpdict/src/ui.rs

use eframe::{egui, epaint, App, Frame};
use crate::dictionary::DictionaryEntry;
use crate::db::search_db;

pub struct DictionaryApp {
    query: String,
    search_results: Vec<DictionaryEntry>
}

impl DictionaryApp {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 加载全局字体
        load_global_font(&cc.egui_ctx);
        DictionaryApp::default()
    }
}

impl Default for DictionaryApp {
    fn default() -> Self {
        Self {
            query: "".to_owned(),
            search_results: Vec::new(),
        }
    }
}

impl App for DictionaryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Japanese Dictionary Search测试");

            ui.text_edit_singleline(&mut self.query)
                .on_hover_text("Enter search query");

            if ui.button("Search").clicked() {
                println!("Searching for: {}", self.query);
                match search_db(&self.query) {
                    Ok(results) => {
                        self.search_results = results;
                        println!("Found {} results", self.search_results.len());
                    }
                    Err(e) => {
                        println!("Error occurred while searching: {:?}", e);
                    }
                }
            }

            ui.label(format!("Found {} results:", self.search_results.len()));

            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in &self.search_results {
                    ui.horizontal(|ui| {
                        ui.label(&entry.word);
                        ui.label(&entry.reading);
                    });
                    ui.label(format!("Translations: {}", entry.translations.join(", ")));
                }
            });
        });
    }
}


/// 全局加载支持中文的字体
pub fn load_global_font(ctx: &egui::Context) {
    let mut fonts = eframe::egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "msyh".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")), // .ttf and .otf supported
    );

    // Put my font first (highest priority):
    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "msyh".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Monospace)
        .unwrap()
        .push("msyh".to_owned());

    ctx.set_fonts(fonts);
}