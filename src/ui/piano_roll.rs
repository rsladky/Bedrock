use super::theme;
use crate::bridge::Bridge;
use crate::project::project::Project;
use crate::ui::UiState;
use eframe::egui::{self, Color32, Pos2, Rect};

const KEYS_WIDTH: f32 = 48.0;
const NOTE_HEIGHT: f32 = 12.0;
const TICKS_PER_BEAT: u32 = 96;
const PIXELS_PER_TICK: f32 = 0.5;
const NUM_OCTAVES: u8 = 8;
const NUM_KEYS: u8 = NUM_OCTAVES * 12;

pub fn show(
    ui: &mut egui::Ui,
    _bridge: &mut Bridge,
    ui_state: &mut UiState,
    project: &mut Project,
) {
    let pattern_idx = ui_state.selected_pattern;

    let total_ticks: u32 = TICKS_PER_BEAT * 16;
    let roll_width = total_ticks as f32 * PIXELS_PER_TICK;
    let roll_height = NUM_KEYS as f32 * NOTE_HEIGHT;

    egui::ScrollArea::both()
        .id_salt("piano_roll_scroll")
        .show(ui, |ui| {
            let (response, painter) = ui.allocate_painter(
                egui::vec2(KEYS_WIDTH + roll_width, roll_height),
                egui::Sense::click_and_drag(),
            );
            let origin = response.rect.min;

            // Draw piano keys sidebar
            for key in 0..NUM_KEYS {
                let y = origin.y + (NUM_KEYS - 1 - key) as f32 * NOTE_HEIGHT;
                let note_in_oct = key % 12;
                let is_black = matches!(note_in_oct, 1 | 3 | 6 | 8 | 10);
                let fill = if is_black {
                    Color32::from_rgb(0x22, 0x22, 0x22)
                } else {
                    Color32::from_rgb(0xcc, 0xcc, 0xcc)
                };
                painter.rect_filled(
                    Rect::from_min_size(
                        Pos2::new(origin.x, y),
                        egui::vec2(KEYS_WIDTH - 2.0, NOTE_HEIGHT - 1.0),
                    ),
                    0.0,
                    fill,
                );
            }

            // Draw grid
            let grid_origin = Pos2::new(origin.x + KEYS_WIDTH, origin.y);
            // Beat lines
            for beat in 0..=16u32 {
                let x = grid_origin.x + beat as f32 * TICKS_PER_BEAT as f32 * PIXELS_PER_TICK;
                let color = if beat % 4 == 0 {
                    Color32::from_rgb(0x55, 0x55, 0x55)
                } else {
                    Color32::from_rgb(0x33, 0x33, 0x33)
                };
                painter.line_segment(
                    [
                        Pos2::new(x, grid_origin.y),
                        Pos2::new(x, grid_origin.y + roll_height),
                    ],
                    egui::Stroke::new(1.0, color),
                );
            }
            // Key lines
            for key in 0..=NUM_KEYS {
                let y = grid_origin.y + key as f32 * NOTE_HEIGHT;
                painter.line_segment(
                    [
                        Pos2::new(grid_origin.x, y),
                        Pos2::new(grid_origin.x + roll_width, y),
                    ],
                    egui::Stroke::new(0.5, Color32::from_rgb(0x2a, 0x2a, 0x2a)),
                );
            }

            // Draw notes
            if let Some(pattern) = project.patterns.get(pattern_idx) {
                for note in &pattern.piano_roll.notes {
                    let y = grid_origin.y + (NUM_KEYS - 1 - note.pitch) as f32 * NOTE_HEIGHT;
                    let x = grid_origin.x + note.start_tick as f32 * PIXELS_PER_TICK;
                    let w = (note.duration_ticks as f32 * PIXELS_PER_TICK).max(4.0);
                    painter.rect_filled(
                        Rect::from_min_size(
                            Pos2::new(x, y + 1.0),
                            egui::vec2(w - 1.0, NOTE_HEIGHT - 2.0),
                        ),
                        2.0,
                        theme::FL_ORANGE,
                    );
                }
            }

            // Click to add note
            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let rel_x = pos.x - grid_origin.x;
                    let rel_y = pos.y - grid_origin.y;
                    if rel_x >= 0.0 && rel_y >= 0.0 {
                        let tick = (rel_x / PIXELS_PER_TICK) as u32;
                        let pitch_idx = (rel_y / NOTE_HEIGHT) as u8;
                        let pitch = (NUM_KEYS - 1).saturating_sub(pitch_idx);
                        if let Some(pattern) = project.patterns.get_mut(pattern_idx) {
                            pattern
                                .piano_roll
                                .notes
                                .push(crate::project::pattern::Note {
                                    channel_id: ui_state.selected_channel,
                                    pitch,
                                    velocity: 100,
                                    start_tick: tick / TICKS_PER_BEAT * TICKS_PER_BEAT, // snap to beat
                                    duration_ticks: TICKS_PER_BEAT,
                                });
                        }
                    }
                }
            }
        });
}
