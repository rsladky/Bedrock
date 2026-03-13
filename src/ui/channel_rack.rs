use eframe::egui::{self, Color32};
use crate::bridge::Bridge;
use crate::ui::UiState;
use crate::project::project::Project;
use crate::project::channel::Channel;
use super::theme;

pub fn show(ui: &mut egui::Ui, _bridge: &mut Bridge, ui_state: &mut UiState, project: &mut Project) {
    ui.heading("Channels");
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        let num_channels = project.channels.len();
        for i in 0..num_channels {
            let ch = &project.channels[i];
            let color = Color32::from_rgb(ch.color[0], ch.color[1], ch.color[2]);
            let selected = ui_state.selected_channel == i;

            let bg = if selected { theme::FL_ORANGE_DIM } else { theme::BG_MID };
            let (rect, response) = ui.allocate_exact_size(
                egui::vec2(ui.available_width() - 4.0, 32.0),
                egui::Sense::click(),
            );
            ui.painter().rect_filled(rect, 3.0, bg);

            // Color swatch
            ui.painter().rect_filled(
                egui::Rect::from_min_size(rect.min + egui::vec2(4.0, 6.0), egui::vec2(6.0, 20.0)),
                1.0, color,
            );

            // Name
            ui.painter().text(
                rect.min + egui::vec2(16.0, 16.0),
                egui::Align2::LEFT_CENTER,
                &ch.name,
                egui::FontId::proportional(12.0),
                theme::TEXT_PRIMARY,
            );

            if response.clicked() {
                ui_state.selected_channel = i;
            }
        }

        ui.separator();

        // Add channel button
        if ui.button("+ Add Channel").clicked() {
            let id = project.channels.len();
            project.channels.push(Channel::new(id, &format!("Ch{}", id + 1), [0x88, 0x88, 0xff]));
            // Resize step grid
            for pattern in &mut project.patterns {
                pattern.step_grid.cells.push(vec![0u8; pattern.step_grid.steps as usize]);
            }
        }
    });
}
