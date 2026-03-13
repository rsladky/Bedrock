use eframe::egui::{self, Color32, Pos2, Stroke};
use std::f32::consts::TAU;

pub struct Knob<'a> {
    value: &'a mut f32,
    range: std::ops::RangeInclusive<f32>,
    size: f32,
    label: Option<String>,
}

impl<'a> Knob<'a> {
    pub fn new(value: &'a mut f32, range: std::ops::RangeInclusive<f32>) -> Self {
        Self { value, range, size: 28.0, label: None }
    }
    pub fn size(mut self, size: f32) -> Self { self.size = size; self }
    pub fn label(mut self, label: impl Into<String>) -> Self { self.label = Some(label.into()); self }
}

impl<'a> egui::Widget for Knob<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, mut response) = ui.allocate_exact_size(
            egui::vec2(self.size, self.size),
            egui::Sense::click_and_drag(),
        );

        if response.dragged() {
            let delta = response.drag_delta().y;
            let range = *self.range.end() - *self.range.start();
            *self.value = (*self.value - delta * range * 0.005)
                .clamp(*self.range.start(), *self.range.end());
            response.mark_changed();
        }

        let painter = ui.painter();
        let center = rect.center();
        let radius = self.size * 0.45;

        // Background circle
        painter.circle_filled(center, radius, Color32::from_rgb(0x30, 0x30, 0x30));
        painter.circle_stroke(center, radius, Stroke::new(1.5, Color32::from_rgb(0x55, 0x55, 0x55)));

        // Value arc
        let normalized = (*self.value - self.range.start()) / (*self.range.end() - *self.range.start());

        // Draw indicator line
        let indicator_len = radius * 0.7;
        let angle = 0.75 * TAU + normalized * 1.5 * TAU;
        let tip = Pos2::new(
            center.x + angle.cos() * indicator_len,
            center.y + angle.sin() * indicator_len,
        );
        painter.line_segment([center, tip], Stroke::new(2.0, super::super::theme::FL_ORANGE));

        response
    }
}
