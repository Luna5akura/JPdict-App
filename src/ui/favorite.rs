/// jpdict/src/ui/favorite.rs

use eframe::egui;
use crate::dictionary::DictionaryEntry;
use crate::ui::DictionaryApp;
use serde::{Deserialize, Serialize};

impl DictionaryApp {
    pub(crate) fn add_to_favorites(&self, entry: DictionaryEntry) {
        let mut favorites = match self.favorites.lock() {
            Ok(fav) => fav,
            Err(_) => {
                eprintln!("Failed to lock favorites mutex");
                return;
            }
        };

        if !favorites.iter().any(|e| *e == entry) {
            favorites.push(entry);
        }
    }

    pub fn show_favorites(&self, ui: &mut egui::Ui) {
        let mut entries_to_remove = Vec::new();

        let entries_for_display: Vec<DictionaryEntry> = {
            let favorites = match self.favorites.lock() {
                Ok(favs) => favs.clone(),
                Err(_) => {
                    ui.label("Failed to load favorites.");
                    return;
                }
            };
            favorites
        };

        if entries_for_display.is_empty() {
            ui.label("No favorites yet.");
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(format!("{} favorite(s):", entries_for_display.len()));
                ui.add_space(10.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (i, entry) in entries_for_display.iter().enumerate() {
                            self.render_card(i, ui, |ui| {
                                ui.horizontal(|ui| {
                                    self.render_search_result_item(ui, entry, i);

                                    if ui.button("x").clicked() {
                                        entries_to_remove.push(entry.clone());
                                    }
                                });
                            });
                        }
                    });
            });
        }

        if !entries_to_remove.is_empty() {
            let mut favorites = self.favorites.lock().unwrap();
            favorites.retain(|e| !entries_to_remove.contains(e));
        }
    }
}
