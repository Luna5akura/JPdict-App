/// jpdict/src/ui/mod.rs

use eframe::{egui, App, Frame};
use crate::dictionary::DictionaryEntry;
use crate::db::search_db;
use arboard::Clipboard;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use super::constants::*;

enum SearchPrompt {
    Query,
    SelectedText,
}

pub struct DictionaryApp {
    query: String,
    search_results: Arc<Mutex<Vec<DictionaryEntry>>>,
    last_clipboard_content: String,
    selected_text: String,
    scroll_to_top: bool,
    previous_char_range: Option<egui::text::CCursorRange>,
    search_thread: Option<std::thread::JoinHandle<()>>,
    runtime: Arc<Runtime>,
    favorites: Arc<Mutex<HashSet<DictionaryEntry>>>,
    showing_favorites: bool,
}

impl Default for DictionaryApp {
    fn default() -> Self {
        let runtime = Runtime::new().unwrap();
        Self {
            query: "".to_owned(),
            search_results: Arc::new(Mutex::new(Vec::new())),
            last_clipboard_content: String::new(),
            selected_text: String::new(),
            scroll_to_top: false,
            previous_char_range: None,
            search_thread: None,
            runtime: Arc::new(runtime),
            favorites: Arc::new(Mutex::new(HashSet::new())),
            showing_favorites: false
        }
    }
}

impl DictionaryApp {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let runtime = Runtime::new().unwrap();
        let mut fonts = eframe::egui::FontDefinitions::default();
        fonts.font_data.insert(
            FONT_NAME.to_owned(),
            // TODO: can the path here changed?
            eframe::egui::FontData::from_static(include_bytes!("../font/epmgobld.ttf")),
            // eframe::egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")),
        );
        fonts
            .families
            .get_mut(&eframe::egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, FONT_NAME.to_owned());
        fonts
            .families
            .get_mut(&eframe::egui::FontFamily::Monospace)
            .unwrap()
            .push(FONT_NAME.to_owned());
        cc.egui_ctx.set_fonts(fonts);
        DictionaryApp::default()
    }

    fn setup_styles(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.text_styles.insert(egui::TextStyle::Body, egui::FontId::new(FONT_SIZE_BODY, egui::FontFamily::Proportional));

        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, OUTLINE_DARK_GRAY);
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(20.0);
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(20.0);
        style.visuals.widgets.active.rounding = egui::Rounding::same(20.0);

        ctx.set_style(style);
    }

    fn add_to_favorites(&self, entry: DictionaryEntry) {
        let mut favorites = self.favorites.lock().unwrap();
        favorites.insert(entry);
    }

    fn show_favorites(&self, ui: &mut egui::Ui) {

        let favorites = self.favorites.lock().unwrap();
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

    fn perform_search(&mut self, prompt: SearchPrompt) {
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

    fn render_search_bar(&mut self, ui: &mut egui::Ui) {
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

    fn render_search_results(&mut self, ui: &mut egui::Ui) {
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

    fn render_search_result_item(&self, ui: &mut egui::Ui, entry: &DictionaryEntry, index: usize) {
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

    fn render_card<R>(&self,cnt: usize, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) {
        egui::Frame::none()
            .fill(CARD_COLORS[cnt % CARD_COLORS.len()])
            .rounding(egui::Rounding::same(20.0))
            .stroke(egui::Stroke::new(1.0, OUTLINE_DARK_GRAY))
            .inner_margin(egui::vec2(10.0, 10.0))
            .shadow(egui::epaint::Shadow {
                offset: egui::vec2(6.0, 6.0),
                blur: 5.0,
                color: egui::Color32::from_black_alpha(30),
                spread: 0.0,
            })
            .show(ui, add_contents);
    }
}

impl App for DictionaryApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        DictionaryApp::setup_styles(ctx);

        let mut clipboard = Clipboard::new().unwrap();
        if let Ok(new_clipboard_content) = clipboard.get_text() {
            if new_clipboard_content != self.last_clipboard_content {
                self.last_clipboard_content = new_clipboard_content.clone();
                self.query = new_clipboard_content;
                self.perform_search(SearchPrompt::Query);
            }
        }

        egui::CentralPanel::default().frame(egui::Frame::window(&ctx.style()).fill(MAIN_LIGHT_BACKGROUND_LIGHT_GRAY)).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::Frame::none().fill(MAIN_LIGHT_BACKGROUND_LIGHT_GRAY).show(ui, |ui| {
                    self.render_search_bar(ui);
                });


                if self.showing_favorites {
                    ui.separator();
                    egui::Frame::none().fill(MAIN_LIGHT_BACKGROUND_LIGHT_GRAY).show(ui, |ui| {
                        self.show_favorites(ui);
                    });
                } else if !self.search_results.lock().unwrap().is_empty() {
                    ui.separator();
                    egui::Frame::none().fill(MAIN_LIGHT_BACKGROUND_LIGHT_GRAY).show(ui, |ui| {
                        self.render_search_results(ui);
                    });
                }


            });
        });
    }
}
