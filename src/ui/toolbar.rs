use super::theme;
use crate::bridge::commands::EngineCommand;
use crate::bridge::Bridge;
use crate::project::project::Project;
use crate::ui::UiState;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, bridge: &mut Bridge, ui_state: &mut UiState, project: &mut Project) {
    ui.horizontal(|ui| {
        ui.visuals_mut().override_text_color = Some(theme::TEXT_PRIMARY);

        // Play / Stop buttons
        let playing = ui_state.snapshot.playing;
        let play_color = if playing {
            theme::FL_ORANGE
        } else {
            theme::BG_LIGHT
        };
        if ui.add(egui::Button::new("▶").fill(play_color)).clicked() {
            if !playing {
                bridge.send(EngineCommand::Play);
                bridge.send(EngineCommand::SetPattern(Box::new(
                    project
                        .patterns
                        .get(ui_state.selected_pattern)
                        .cloned()
                        .unwrap_or_default(),
                )));
            }
        }
        if ui
            .add(egui::Button::new("■").fill(theme::BG_LIGHT))
            .clicked()
        {
            bridge.send(EngineCommand::Stop);
        }

        ui.separator();

        // BPM
        ui.label("BPM:");
        let mut bpm = project.bpm;
        if ui
            .add(
                egui::DragValue::new(&mut bpm)
                    .range(40.0..=300.0)
                    .speed(0.5),
            )
            .changed()
        {
            project.bpm = bpm;
            bridge.send(EngineCommand::SetBpm(bpm));
        }

        ui.separator();

        // Time sig
        let (num, den) = project.time_signature;
        ui.label(format!("{}/{}", num, den));

        ui.separator();

        // CPU
        let cpu = ui_state.snapshot.cpu_load;
        ui.label(format!("CPU: {:.0}%", cpu * 100.0));

        ui.separator();

        // Peak meters
        if let (Some(&l), Some(&r)) = (
            ui_state.snapshot.peak_levels.get(0),
            ui_state.snapshot.peak_levels.get(1),
        ) {
            draw_peak_meter(ui, l);
            draw_peak_meter(ui, r);
        }
    });
}

fn draw_peak_meter(ui: &mut egui::Ui, level: f32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(8.0, 16.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, theme::BG_DARK);
    let fill_h = rect.height() * level.min(1.0);
    let fill_rect = egui::Rect::from_min_size(
        egui::pos2(rect.min.x, rect.max.y - fill_h),
        egui::vec2(rect.width(), fill_h),
    );
    let color = if level > 0.9 {
        theme::RED
    } else if level > 0.7 {
        theme::YELLOW
    } else {
        theme::GREEN
    };
    painter.rect_filled(fill_rect, 0.0, color);
}
