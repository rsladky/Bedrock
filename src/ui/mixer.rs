use eframe::egui::{self, Color32};
use crate::bridge::Bridge;
use crate::bridge::commands::EngineCommand;
use crate::ui::UiState;
use crate::project::project::Project;
use super::theme;

pub fn show(ui: &mut egui::Ui, bridge: &mut Bridge, _ui_state: &mut UiState, project: &mut Project) {
    ui.horizontal(|ui| {
        let num_channels = project.mixer.len();
        for i in 0..num_channels {
            let ch = &mut project.mixer[i];
            ui.vertical(|ui| {
                ui.set_min_width(50.0);
                // Channel name
                ui.label(egui::RichText::new(&ch.name).size(10.0).color(theme::TEXT_DIM));

                // VU meter placeholder
                ui.add_space(4.0);
                let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 60.0), egui::Sense::hover());
                ui.painter().rect_filled(rect, 0.0, theme::BG_DARK);

                // Fader
                let mut vol = ch.volume;
                let prev_vol = vol;
                ui.add(egui::Slider::new(&mut vol, 0.0f32..=1.5).vertical().show_value(false));
                if (vol - prev_vol).abs() > 1e-6 {
                    ch.volume = vol;
                    bridge.send(EngineCommand::SetMixerVolume { channel_idx: i, volume: vol });
                }

                // Pan knob (simplified as drag)
                let mut pan = ch.pan;
                ui.add(egui::DragValue::new(&mut pan).range(-1.0f32..=1.0).speed(0.01).prefix("P:"));
                ch.pan = pan;

                // Mute
                let mute_color = if ch.muted { Color32::from_rgb(0xff, 0x44, 0x00) } else { theme::BG_LIGHT };
                if ui.add(egui::Button::new("M").fill(mute_color).min_size(egui::vec2(20.0, 16.0))).clicked() {
                    ch.muted = !ch.muted;
                }
            });
            ui.separator();
        }
    });
}
