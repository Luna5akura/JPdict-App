#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/// jpdict/src/ui.rs

use eframe::{egui, App, Frame};
use crate::dictionary::DictionaryEntry;
use crate::db::search_db;

pub struct DictionaryApp {
    query: String,
    search_results: Vec<DictionaryEntry>,
    bg_colors: Vec<egui::Color32>,
}

impl DictionaryApp {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = eframe::egui::FontDefinitions::default();
        fonts.font_data.insert(
            "msyh".to_owned(),
            eframe::egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")),
        );
        fonts
            .families
            .get_mut(&eframe::egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "msyh".to_owned());
        fonts
            .families
            .get_mut(&eframe::egui::FontFamily::Monospace)
            .unwrap()
            .push("msyh".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        DictionaryApp::default()
    }
}

impl Default for DictionaryApp {
    fn default() -> Self {
        Self {
            query: "".to_owned(),
            search_results: Vec::new(),
            bg_colors: vec![
                egui::Color32::from_rgb(240, 248, 255), // Alice Blue
                egui::Color32::from_rgb(250, 235, 215), // Antique White
                egui::Color32::from_rgb(230, 230, 250), // Lavender
                egui::Color32::from_rgb(255, 228, 225), // Misty Rose
                egui::Color32::from_rgb(240, 255, 255), // Azure
                egui::Color32::from_rgb(245, 245, 220), // Beige
            ],
        }
    }
}

impl App for DictionaryApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {

        egui::CentralPanel::default().frame(egui::Frame::window(&ctx.style()).fill(egui::Color32::from_rgb(240, 240, 240))).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Japanese Dictionary Search");
                ui.separator();

                egui::Frame::none().fill(egui::Color32::from_rgb(250, 250, 250)).show(ui, |ui| {
                    self.render_search_bar(ui);
                });

                if !self.search_results.is_empty() {
                    ui.separator();
                    egui::Frame::none().fill(egui::Color32::from_rgb(245, 245, 245)).show(ui, |ui| {
                        self.render_search_results(ui);
                    });
                }
            });
        });
    }
}

impl DictionaryApp {
    fn render_search_bar(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            let search_response = ui.text_edit_singleline(&mut self.query);

            if ui.button("Search").clicked() || search_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                match search_db(&self.query, 0, 20) {
                    Ok(results) => {
                        self.search_results = results;
                        println!("Found {} results", self.search_results.len());
                    }
                    Err(e) => {
                        println!("Error occurred while searching: {:?}", e);
                    }
                }
            }

            if ui.button("❌").clicked() {
                self.query.clear();
            }
        });
    }

    fn render_search_results(&self, ui: &mut egui::Ui) {
        ui.label(format!("Found {} results:", self.search_results.len()));
        ui.add_space(10.0);

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for (i, entry) in self.search_results.iter().enumerate() {
                    egui::Frame::none().fill(self.bg_colors[i % self.bg_colors.len()]).show(ui, |ui| {
                        self.render_search_result_item(ui, entry, i);
                    });
                    ui.add_space(20.0);
                }
            });

        ui.separator();
    }

    fn render_search_result_item(&self, ui: &mut egui::Ui, entry: &DictionaryEntry, index: usize) {
        ui.vertical_centered(|ui| {
            ui.group(|ui| {
                ui.set_width(600.0);

                ui.style_mut().override_text_style = Some(egui::TextStyle::Body);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&entry.word).size(40.0).strong()).on_hover_text(format!("Pronunciation: {}", entry.pronunciation));
                    ui.label(format!("【{}】", &entry.reading));
                });

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("{}", entry.pos)).color(egui::Color32::LIGHT_BLUE));

                    if let Some(infl) = &entry.inflection {
                        ui.label(egui::RichText::new(format!("({})", infl)).color(egui::Color32::LIGHT_GREEN));
                    }

                    if let Some(tags) = &entry.tags {
                        ui.horizontal_wrapped(|ui| {
                            for tag in tags.split(' ') {
                                ui.label(
                                    egui::RichText::new(tag)
                                        .background_color(egui::Color32::from_rgb(224, 240, 255))
                                        .color(egui::Color32::from_rgb(0, 123, 255))
                                ).on_hover_text("Tag explanation here");
                                ui.add_space(5.0);
                            }
                        });
                    }
                }).response.on_hover_text(format!("Frequency: {}", entry.freq));

                ui.add_space(10.0);

                ui.label("Translations:");
                ui.vertical(|ui| {
                    for tran in &entry.translations {
                        ui.label(format!("- {}", tran));
                    }
                });
            });
        });
    }
}