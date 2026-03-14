use super::super::theme;
use eframe::egui::{self, Color32};

pub struct Led {
    active: bool,
    playing: bool,
    size: f32,
}

impl Led {
    pub fn new(active: bool, playing: bool) -> Self {
        Self {
            active,
            playing,
            size: 24.0,
        }
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }
}

impl egui::Widget for Led {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(self.size, self.size), egui::Sense::click());
        let color = if self.playing {
            theme::FL_ORANGE
        } else if self.active {
            theme::FL_ORANGE_DIM
        } else {
            Color32::from_rgb(0x35, 0x35, 0x35)
        };
        ui.painter().rect_filled(rect, 4.0, color);
        ui.painter().rect_stroke(
            rect,
            4.0,
            egui::Stroke::new(1.0, Color32::from_rgb(0x55, 0x55, 0x55)),
        );
        response
    }
}
