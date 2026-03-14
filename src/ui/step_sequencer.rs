use super::theme;
use crate::bridge::commands::EngineCommand;
use crate::bridge::Bridge;
use crate::project::project::Project;
use crate::ui::UiState;
use eframe::egui::{self, Color32};

pub fn show(ui: &mut egui::Ui, bridge: &mut Bridge, ui_state: &mut UiState, project: &mut Project) {
    let pattern_idx = ui_state.selected_pattern;
    let Some(pattern) = project.patterns.get_mut(pattern_idx) else {
        return;
    };
    let num_channels = project.channels.len();
    let num_steps = pattern.step_grid.steps as usize;
    let active_step = ui_state.snapshot.active_step;
    let playing = ui_state.snapshot.playing;

    let cell_size = egui::vec2(32.0, 28.0);
    let spacing = 2.0;

    egui::ScrollArea::vertical().show(ui, |ui| {
        for ch_idx in 0..num_channels {
            ui.horizontal(|ui| {
                // Channel label
                let ch = &project.channels[ch_idx];
                let color = Color32::from_rgb(ch.color[0], ch.color[1], ch.color[2]);
                ui.colored_label(color, format!("{:<8}", ch.name));

                for step in 0..num_steps {
                    let velocity = project
                        .patterns
                        .get(pattern_idx)
                        .and_then(|p| p.step_grid.cells.get(ch_idx))
                        .and_then(|r| r.get(step))
                        .copied()
                        .unwrap_or(0);
                    let is_active = velocity > 0;
                    let is_playing_step = playing && step == active_step;

                    let fill = if is_playing_step {
                        theme::FL_ORANGE
                    } else if is_active {
                        theme::FL_ORANGE_DIM
                    } else if step % 4 == 0 {
                        Color32::from_rgb(0x30, 0x30, 0x30)
                    } else {
                        theme::BG_LIGHT
                    };

                    let (rect, response) = ui.allocate_exact_size(cell_size, egui::Sense::click());
                    ui.painter().rect_filled(rect, 3.0, fill);
                    ui.painter().rect_stroke(
                        rect,
                        3.0,
                        egui::Stroke::new(1.0, Color32::from_rgb(0x55, 0x55, 0x55)),
                    );

                    if response.clicked() {
                        if let Some(pattern2) = project.patterns.get_mut(pattern_idx) {
                            pattern2.step_grid.toggle(ch_idx, step);
                        }
                        // Send updated pattern
                        if let Some(p) = project.patterns.get(pattern_idx) {
                            bridge.send(EngineCommand::SetPattern(Box::new(p.clone())));
                        }
                    }

                    ui.add_space(spacing);
                }
            });
            ui.add_space(spacing);
        }
    });
}
