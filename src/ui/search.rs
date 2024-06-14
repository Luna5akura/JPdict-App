/// jpdict/src/ui/search.rs

use eframe::egui;
use crate::db::search_db;
use crate::dictionary::DictionaryEntry;
use crate::ui::constants::*;
use crate::ui::DictionaryApp;

pub enum SearchPrompt {
    Query,
    SelectedText,
}


impl DictionaryApp {

    pub fn perform_search(&mut self, prompt: SearchPrompt) {
        self.showing_favorites = false;
        let search_text = match prompt {
            SearchPrompt::Query => self.query.clone(),
            SearchPrompt::SelectedText => self.selected_text.clone(),
        };

        let search_results = self.search_results.clone();
        let runtime = self.runtime.clone();

        runtime.spawn(async move {
            match search_db(&search_text, 0, 20).await {
                Ok(results) => {
                    *search_results.lock().unwrap() = results;
                }
                Err(e) => {
                    println!("Error occurred while searching: {:?}", e);
                }
            }
        });
    }

    pub fn render_search_bar(&mut self, ui: &mut egui::Ui) {
        let mut search_triggered = false;
        let mut selection_changed = false;

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let total_width = ui.available_width();
            let element_width = 10.0 + 300.0 + 10.0 + 100.0 + 10.0 + 40.0 + 10.0 + 40.0 + 10.0;
            let remaining_space = total_width - element_width;
            let side_space = remaining_space / 2.0;

            ui.add_space(side_space);

            let search_bar = egui::TextEdit::multiline(&mut self.query)
                .font(egui::TextStyle::Body)
                .frame(true)
                .desired_width(300.0)
                .margin(egui::vec2(15.0, 10.0));

            let search_bar_output = search_bar.show(ui);

            let search_response = search_bar_output.response;
            let search_text_cursor = search_bar_output.state.cursor;

            if let Some(current_char_range) = search_text_cursor.char_range() {
                if self.previous_char_range != Some(current_char_range) {
                    self.previous_char_range = Some(current_char_range);

                    let sorted_cursors = current_char_range.sorted();
                    let start = sorted_cursors[0].index;
                    let end = sorted_cursors[1].index;

                    let char_indices: Vec<_> = self.query.char_indices().collect();
                    let start_char_index = if start < char_indices.len() {
                        char_indices[start].0
                    } else {
                        self.query.len()
                    };

                    let end_char_index = if end < char_indices.len() {
                        char_indices[end].0
                    } else {
                        self.query.len()
                    };

                    self.selected_text = self.query[start_char_index..end_char_index].to_string();

                    if !(self.selected_text.chars().all(|c| c.is_ascii_alphabetic()) && end - start < 3) {
                        selection_changed = true;
                    }

                    println!("Selected text: {}", self.selected_text);
                }
            } else {
                if self.previous_char_range.is_some() {
                    selection_changed = true;
                    self.previous_char_range = None;
                    self.selected_text.clear();
                    println!("Selection cleared")
                }
            }

            if ui.add_sized(
                [100.0, 35.0],
                egui::Button::new(egui::RichText::new("Search").size(FONT_SIZE_MEDIUM))
            ).clicked()
                || (search_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                search_triggered = true;
            }

            if ui.add_sized([40.0, 35.0], egui::Button::new(
                egui::RichText::new("×").size(FONT_SIZE_MEDIUM)
            )).clicked() {
                self.query.clear();
            }

            // Button to show favorites
            if ui.add_sized(
                [40.0, 35.0],
                egui::Button::new(egui::RichText::new("★").size(FONT_SIZE_MEDIUM))
            ).clicked() {
                self.showing_favorites = true;
            }
            ui.add_space(side_space);
        });

        ui.add_space(10.0);
        if selection_changed {
            self.perform_search(SearchPrompt::SelectedText);
            println!("Selection changed: {}", self.selected_text)
        }
        if search_triggered {
            self.perform_search(SearchPrompt::Query)
        }
    }

    pub fn render_search_results(&mut self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        ui.label(format!("Found {} results:", self.search_results.lock().unwrap().len()));
        ui.add_space(10.0);

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if self.scroll_to_top {
                    ui.scroll_to_cursor(Some(egui::Align::Min));
                    self.scroll_to_top = false;
                }
                let search_results = self.search_results.lock().unwrap();
                for (i, entry) in search_results.iter().enumerate() {
                    self.render_card(i, ui, |ui| {
                        self.render_search_result_item(ui, entry, i);
                    });
                    ui.add_space(20.0);
                }
            });

        ui.separator();
    }

    pub fn render_search_result_item(&self, ui: &mut egui::Ui, entry: &DictionaryEntry, index: usize) {
        ui.vertical_centered(|ui| {
            ui.set_width(600.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(&entry.word).size(FONT_SIZE_LARGE).strong()).on_hover_text(format!("Pronunciation: {}", entry.pronunciation));
                    ui.label(egui::RichText::new(format!("【{}】", &entry.reading)).size(FONT_SIZE_MEDIUM));
                });

                // Add favorite button
                if ui.button("★").on_hover_text("Add to favorites").clicked() {
                    self.add_to_favorites(entry.clone());
                }
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("{}", entry.pos))
                    .size(15.0).strong().color(PROPERTY_COLOR_BLUE));

                if let Some(infl) = &entry.inflection {
                    ui.label(egui::RichText::new(format!("({})", infl))
                        .size(15.0).strong().color(ADDITION_COLOR_DARKGREEN));
                }

                if let Some(tags) = &entry.tags {
                    ui.horizontal_wrapped(|ui| {
                        for tag in tags.split(' ') {
                            ui.label(
                                egui::RichText::new(tag)
                                    .size(FONT_SIZE_SMALL).strong()
                                    .background_color(TAG_BACKGROUND_LIGHT_BLUE)
                                    .color(TAG_COLOR_BLUE)
                            ).on_hover_text("Tag explanation here");
                            ui.add_space(5.0);
                        }
                    });
                }
            }).response.on_hover_text(format!("Frequency: {}", entry.freq));

            ui.add_space(10.0);

            ui.vertical(|ui| {
                // ui.label(egui::RichText::new("Translations:").size(FONT_SIZE_MEDIUM).strong());
                for tran in &entry.translations {
                    ui.label(egui::RichText::new(format!("- {}", tran)).size(FONT_SIZE_MEDIUM).strong());
                    // ui.label(egui::RichText::new(format!("- {}", tran)).size(FONT_SIZE_SMALL));
                }
            });
        });
    }

}
