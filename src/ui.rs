#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/// jpdict/src/ui.rs

use eframe::{egui, App, Frame};
use crate::dictionary::DictionaryEntry;
use crate::db::search_db;

pub struct DictionaryApp {
    query: String,
    search_results: Vec<DictionaryEntry>
}

impl DictionaryApp {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
            ui.heading("Japanese Dictionary Search");
            ui.horizontal(|ui| {
                let search_edit = ui.text_edit_singleline(&mut self.query)
                    .on_hover_text("Enter search query");
                // if search_edit.lost_focus() && ui.input(()).key_pressed(egui::Key::Enter) {
                //     match search_db(&self.query, 0, 20) {
                //         Ok(results) => {
                //             self.search_results = results;
                //             println!("Found {} results", self.search_results.len());
                //         }
                //         Err(e) => {
                //             println!("Error occurred while searching: {:?}", e);
                //         }
                //     }
                // }
                if ui.button("🔍").clicked() {
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

            ui.label(format!("Found {} results:", self.search_results.len()));

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, entry) in self.search_results.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}. ", i+1)).on_hover_text(format!("Sequence: {}", entry.sequence));
                        ui.strong(&entry.word);
                        ui.label(format!("【{}】", &entry.reading)).on_hover_text(format!("Pronunciation: {}", entry.pronunciation));
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("{}", entry.pos));
                        if let Some(infl) = &entry.inflection {
                            ui.label(format!("({})", infl));
                        }
                        if let Some(tags) = &entry.tags {
                            ui.label(format!("· {}", tags));
                        }
                    }).response.on_hover_text(format!("Frequency: {}", entry.freq));

                    ui.label(format!("Translations:"));
                    for tran in &entry.translations {
                        ui.label(format!("- {}", tran));
                    }

                    ui.add_space(10.0);
                }
            });
        });
    }
}


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