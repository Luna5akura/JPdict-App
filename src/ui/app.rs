/// jpdict/src/ui/app.rs

use eframe::{egui, App, Frame};
use crate::dictionary::DictionaryEntry;
use arboard::Clipboard;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
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
    pub(crate) favorites: Arc<Mutex<HashSet<DictionaryEntry>>>,
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




    pub fn render_card<R>(&self,cnt: usize, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) {
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
        setup_styles(ctx);

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
