/// jpdict/src/ui.rs

use eframe::{egui, App, Frame};
use crate::dictionary::DictionaryEntry;
use crate::db::search_db;
use arboard::Clipboard;

// 颜色常量
const CARD_0_ALICE_BLUE: egui::Color32 = egui::Color32::from_rgb(240, 248, 255);
const CARD_1_ANTIQUE_WHITE: egui::Color32 = egui::Color32::from_rgb(250, 235, 215);
const CARD_2_LAVENDER: egui::Color32 = egui::Color32::from_rgb(230, 230, 250);
const CARD_3_MISTY_ROSE: egui::Color32 = egui::Color32::from_rgb(255, 228, 225);
const CARD_4_AZURE: egui::Color32 = egui::Color32::from_rgb(240, 255, 255);
const CARD_5_BEIGE: egui::Color32 = egui::Color32::from_rgb(245, 245, 220);
const MAIN_LIGHT_BACKGROUND_LIGHT_GRAY: egui::Color32 = egui::Color32::from_rgb(240, 240, 240);
const WHITE_SMOKE: egui::Color32 = egui::Color32::from_rgb(250, 250, 250);
const GAINSBORO: egui::Color32 = egui::Color32::from_rgb(245, 245, 245);
const TAG_BACKGROUND_LIGHT_BLUE: egui::Color32 = egui::Color32::from_rgb(224, 240, 255);
const TAG_COLOR_BLUE: egui::Color32 = egui::Color32::from_rgb(0, 123, 255);
const OUTLINE_DARK_GRAY: egui::Color32 = egui::Color32::DARK_GRAY;
const PROPERTY_COLOR_BLUE: egui::Color32 = egui::Color32::BLUE;
const ADDITION_COLOR_DARKGREEN: egui::Color32 = egui::Color32::DARK_GREEN;
// const FONT_NAME: &str = "A-OTF-GothicMB101Pr5-Reg";
const FONT_NAME: &str = "epmgobld";

const FONT_SIZE_BODY: f32 = 20.0;
const FONT_SIZE_LARGE: f32 = 40.0;
const FONT_SIZE_MEDIUM: f32 = 20.0;
const FONT_SIZE_SMALL: f32 = 15.0;

pub struct DictionaryApp {
    query: String,
    search_results: Vec<DictionaryEntry>,
    bg_colors: Vec<egui::Color32>,
    last_clipboard_content: String,
    selected_text: String,
}

impl DictionaryApp {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = eframe::egui::FontDefinitions::default();
        fonts.font_data.insert(
            FONT_NAME.to_owned(),
            // TODO: can the path here changed?
            eframe::egui::FontData::from_static(include_bytes!("./font/epmgobld.ttf")),
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
}

impl Default for DictionaryApp {
    fn default() -> Self {
        Self {
            query: "".to_owned(),
            search_results: Vec::new(),
            bg_colors: vec![CARD_0_ALICE_BLUE, CARD_1_ANTIQUE_WHITE, CARD_2_LAVENDER, CARD_3_MISTY_ROSE, CARD_4_AZURE, CARD_5_BEIGE],
            last_clipboard_content: String::new(),
            selected_text: String::new(),
        }
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
                self.perform_search();
            }
        }

        egui::CentralPanel::default().frame(egui::Frame::window(&ctx.style()).fill(MAIN_LIGHT_BACKGROUND_LIGHT_GRAY)).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::Frame::none().fill(MAIN_LIGHT_BACKGROUND_LIGHT_GRAY).show(ui, |ui| {
                    self.render_search_bar(ui);
                });

                if !self.search_results.is_empty() {
                    ui.separator();
                    egui::Frame::none().fill(MAIN_LIGHT_BACKGROUND_LIGHT_GRAY).show(ui, |ui| {
                        self.render_search_results(ui);
                    });
                }
            });
        });
    }
}

impl DictionaryApp {
    fn perform_search(&mut self) {
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
    fn render_search_bar(&mut self, ui: &mut egui::Ui) {
        let mut search_triggered = false;
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let total_width = ui.available_width();
            let element_width = 10.0 + 300.0 + 10.0 + 100.0 + 10.0 + 40.0 + 10.0;
            let remaining_space = total_width - element_width;
            let side_space = remaining_space / 2.0;

            ui.add_space(side_space);

            let search_response = ui.add(
                egui::TextEdit::singleline(&mut self.query)
                    .font(egui::TextStyle::Body)
                    .frame(true)
                    .desired_width(300.0)
                    .margin(egui::vec2(15.0, 10.0))
            );

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

            ui.add_space(side_space);
        });

        ui.add_space(10.0);

        if search_triggered {
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
    }

    fn render_search_results(&self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        ui.label(format!("Found {} results:", self.search_results.len()));
        ui.add_space(10.0);

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for (i, entry) in self.search_results.iter().enumerate() {
                    egui::Frame::none()
                        .fill(self.bg_colors[i % self.bg_colors.len()])
                        .rounding(egui::Rounding::same(20.0))
                        .stroke(egui::Stroke::new(1.0, OUTLINE_DARK_GRAY))
                        .inner_margin(egui::vec2(10.0, 10.0))
                        .shadow(egui::epaint::Shadow {
                            offset: egui::vec2(6.0, 6.0),
                            blur: 5.0,
                            color: egui::Color32::from_black_alpha(30),
                            spread: 0.0,
                        })
                        .show(ui, |ui| {

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

            ui.vertical(|ui| {
                ui.label(egui::RichText::new(&entry.word).size(FONT_SIZE_LARGE).strong()).on_hover_text(format!("Pronunciation: {}", entry.pronunciation));
                ui.label(egui::RichText::new(format!("【{}】", &entry.reading)).size(FONT_SIZE_MEDIUM));
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