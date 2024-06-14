use eframe::egui;

pub const CARD_COLORS: [egui::Color32; 6] = [
    egui::Color32::from_rgb(240, 248, 255),  // ALICE_BLUE
    egui::Color32::from_rgb(250, 235, 215),  // ANTIQUE_WHITE
    egui::Color32::from_rgb(230, 230, 250),  // LAVENDER
    egui::Color32::from_rgb(255, 228, 225),  // MISTY_ROSE
    egui::Color32::from_rgb(240, 255, 255),  // AZURE
    egui::Color32::from_rgb(245, 245, 220),  // BEIGE
];
pub const MAIN_LIGHT_BACKGROUND_LIGHT_GRAY: egui::Color32 = egui::Color32::from_rgb(240, 240, 240);
pub const WHITE_SMOKE: egui::Color32 = egui::Color32::from_rgb(250, 250, 250);
pub const GAINSBORO: egui::Color32 = egui::Color32::from_rgb(245, 245, 245);
pub const TAG_BACKGROUND_LIGHT_BLUE: egui::Color32 = egui::Color32::from_rgb(224, 240, 255);
pub const TAG_COLOR_BLUE: egui::Color32 = egui::Color32::from_rgb(0, 123, 255);
pub const OUTLINE_DARK_GRAY: egui::Color32 = egui::Color32::DARK_GRAY;
pub const PROPERTY_COLOR_BLUE: egui::Color32 = egui::Color32::BLUE;
pub const ADDITION_COLOR_DARKGREEN: egui::Color32 = egui::Color32::DARK_GREEN;
// const FONT_NAME: &str = "A-OTF-GothicMB101Pr5-Reg";
pub const FONT_NAME: &str = "epmgobld";

pub const FONT_SIZE_BODY: f32 = 20.0;
pub const FONT_SIZE_LARGE: f32 = 40.0;
pub const FONT_SIZE_MEDIUM: f32 = 20.0;
pub const FONT_SIZE_SMALL: f32 = 15.0;