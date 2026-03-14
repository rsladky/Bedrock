use eframe::egui::{self, Color32, Pos2, Rect};

pub struct Fader<'a> {
    value: &'a mut f32,
    range: std::ops::RangeInclusive<f32>,
    width: f32,
    height: f32,
}

impl<'a> Fader<'a> {
    pub fn new(value: &'a mut f32) -> Self {
        Self {
            value,
            range: 0.0..=1.5,
            width: 20.0,
            height: 80.0,
        }
    }
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl<'a> egui::Widget for Fader<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, mut response) = ui.allocate_exact_size(
            egui::vec2(self.width, self.height),
            egui::Sense::click_and_drag(),
        );

        if response.dragged() {
            let delta = response.drag_delta().y;
            let range = *self.range.end() - *self.range.start();
            *self.value = (*self.value - delta * range / self.height)
                .clamp(*self.range.start(), *self.range.end());
            response.mark_changed();
        }

        let painter = ui.painter();
        // Track
        painter.rect_filled(rect, 2.0, Color32::from_rgb(0x18, 0x18, 0x18));

        // Thumb position
        let normalized =
            (*self.value - self.range.start()) / (*self.range.end() - self.range.start());
        let thumb_y = rect.max.y - normalized * rect.height();
        let thumb_rect = Rect::from_center_size(
            Pos2::new(rect.center().x, thumb_y),
            egui::vec2(self.width, 8.0),
        );
        let thumb_color = if response.hovered() || response.dragged() {
            Color32::from_rgb(0xcc, 0xcc, 0xcc)
        } else {
            Color32::from_rgb(0x88, 0x88, 0x88)
        };
        painter.rect_filled(thumb_rect, 2.0, thumb_color);

        response
    }
}
