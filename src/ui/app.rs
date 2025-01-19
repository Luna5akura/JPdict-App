/// jpdict/src/ui/app.rs

use eframe::{egui, App, Frame};
use crate::dictionary::DictionaryEntry;
use arboard::Clipboard;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use super::constants::*;
use super::styles::setup_styles;
use super::search::SearchPrompt;

pub struct DictionaryApp {
    pub(crate) query: String,
    pub(crate) search_results: Arc<Mutex<Vec<DictionaryEntry>>>,
    pub(crate) last_clipboard_content: String,
    pub(crate) selected_text: String,
    pub(crate) scroll_to_top: bool,
    pub(crate) previous_char_range: Option<egui::text::CCursorRange>,
    pub(crate) runtime: Arc<Runtime>,
    pub(crate) favorites: Arc<Mutex<Vec<DictionaryEntry>>>,
    pub(crate) showing_favorites: bool,
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
            runtime: Arc::new(runtime),
            favorites: Arc::new(Mutex::new(Vec::new())),
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

        let mut app = DictionaryApp::default();
        app.load_favorites();
        app
    }

    fn save_favorites(&self) {
        let favorites = self.favorites.lock().unwrap();
        let path = "favorites.json";
        match serde_json::to_string(&*favorites) {
            Ok(json) => {
                if let Err(e) = std::fs::write(path, json) {
                    eprintln!("Failed to save favorites: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Failed to serialize favorites: {}", e);
            }
        }
    }

    fn load_favorites(&mut self) {
        let path = "favorites.json";
        if let Ok(json) = std::fs::read_to_string(path) {
            match serde_json::from_str(&json) {
                Ok(favs) => {
                    *self.favorites.lock().unwrap() = favs;
                },
                Err(e) => {
                    eprintln!("Failed to deserialize favorites: {}", e);
                }
            }
        }
    }

    pub fn render_card<R>(&self,cnt: usize, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) {
        let is_dark_mode = ui.style().visuals.dark_mode;

        let card_colors = if is_dark_mode {
            DARK_CARD_COLORS
        } else {
            LIGHT_CARD_COLORS
        };

        egui::Frame::none()
            .fill(card_colors[cnt % card_colors.len()])
            .rounding(egui::Rounding::same(20.0))
            .stroke(egui::Stroke::new(1.0, LIGHT_OUTLINE_DARK_GRAY))
            .inner_margin(egui::vec2(10.0, 10.0))
            .shadow(egui::epaint::Shadow {
                offset: egui::vec2(6.0, 6.0),
                blur: 5.0,
                color: egui::Color32::from_black_alpha(30),
                spread: 0.0,
            })
            .show(ui, add_contents);
    }

    pub fn set_dark_mode(&self) {
        self.runtime.spawn(async {
            egui::Context::set_visuals(&egui::Context::default(), egui::Visuals {
                dark_mode: true,
                ..egui::Visuals::default()
            });
        });
    }

    pub fn set_light_mode(&self) {
        self.runtime.spawn(async {
            egui::Context::set_visuals(&egui::Context::default(), egui::Visuals {
                dark_mode: false,
                ..egui::Visuals::default()
            });
        });
    }
}

impl App for DictionaryApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        setup_styles(ctx);

        let main_background = if ctx.style().visuals.dark_mode {
            DARK_MAIN_BACKGROUND_GRAY
        } else {
            LIGHT_MAIN_BACKGROUND_LIGHT_GRAY
        };

        let mut clipboard = Clipboard::new().unwrap();
        if let Ok(new_clipboard_content) = clipboard.get_text() {
            if new_clipboard_content != self.last_clipboard_content {
                self.last_clipboard_content = new_clipboard_content.clone();
                self.query = new_clipboard_content;
                self.perform_search(SearchPrompt::Query);
            }
        }

        egui::CentralPanel::default().frame(egui::Frame::window(&ctx.style()).fill(main_background)).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::Frame::none().fill(main_background).show(ui, |ui| {
                    self.render_search_bar(ui);
                });

                if self.showing_favorites {
                    ui.separator();
                    egui::Frame::none().fill(main_background).show(ui, |ui| {
                        self.show_favorites(ui);
                    });
                } else if !self.search_results.lock().unwrap().is_empty() {
                    ui.separator();
                    egui::Frame::none().fill(main_background).show(ui, |ui| {
                        self.render_search_results(ui);
                    });
                }
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_favorites();
    }
}
