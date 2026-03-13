use eframe::egui::{self, Color32};

pub struct PianoKey {
    pitch: u8,
    active: bool,
}

impl PianoKey {
    pub fn new(pitch: u8, active: bool) -> Self {
        Self { pitch, active }
    }
}

impl egui::Widget for PianoKey {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let is_black = matches!(self.pitch % 12, 1 | 3 | 6 | 8 | 10);
        let (w, h) = if is_black { (28.0, 8.0) } else { (40.0, 12.0) };
        let (rect, response) = ui.allocate_exact_size(egui::vec2(w, h), egui::Sense::click());

        let color = if self.active {
            Color32::from_rgb(0xff, 0x78, 0x00)
        } else if is_black {
            Color32::from_rgb(0x22, 0x22, 0x22)
        } else {
            Color32::from_rgb(0xcc, 0xcc, 0xcc)
        };
        ui.painter().rect_filled(rect, 1.0, color);
        ui.painter().rect_stroke(rect, 1.0, egui::Stroke::new(0.5, Color32::from_rgb(0x55, 0x55, 0x55)));

        response
    }
}
