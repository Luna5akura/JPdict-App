/// jpdict/src/ui.rs

use eframe::{egui, epaint, App, Frame};
use crate::dictionary::DictionaryEntry;
use crate::db::search_db;

pub struct DictionaryApp {
    query: String,
}

impl Default for DictionaryApp {
    fn default() -> Self {
        Self {
            query: "".to_owned(),
        }
    }
}

impl App for DictionaryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Japanese Dictionary Search");

            ui.text_edit_singleline(&mut self.query)
                .on_hover_text("Enter search query");

            if ui.button("Search").clicked() {
                if let Ok(results) = search_db(&self.query) {
                    ui.label(format!("Found {} results:", results.len()));

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for entry in results {
                            ui.horizontal(|ui| {
                                ui.label(&entry.word);
                                ui.label(&entry.reading);
                            });
                            ui.label(format!("Translations: {}", entry.translations.join(", ")));
                            ui.end_row();
                        }
                    });
                } else {
                    ui.label("Error occurred while searching.");
                }
            }
        });
    }
}
