/// jpdict/src/ui/favorite.rs

use eframe::egui;
use crate::dictionary::DictionaryEntry;
use crate::ui::DictionaryApp;

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
        let favorites = match self.favorites.lock() {
            Ok(fav) => fav,
            Err(_) => {
                ui.label("Failed to load favorites.");
                return;
            }
        };

        if favorites.is_empty() {
            ui.label("No favorites yet.");
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(format!("{} favorite(s):", favorites.len()));
                ui.add_space(10.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (i, entry) in favorites.iter().enumerate() {
                            self.render_card(i, ui, |ui| {
                                self.render_search_result_item(ui, entry, i);
                            });
                        }
                    });
            });
        }
    }
}
