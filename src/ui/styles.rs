use eframe::egui;
use crate::ui::constants::{FONT_SIZE_BODY, OUTLINE_DARK_GRAY};

pub fn setup_styles(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.text_styles.insert(egui::TextStyle::Body, egui::FontId::new(FONT_SIZE_BODY, egui::FontFamily::Proportional));

        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, OUTLINE_DARK_GRAY);
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(20.0);
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(20.0);
        style.visuals.widgets.active.rounding = egui::Rounding::same(20.0);

        ctx.set_style(style);
    }

