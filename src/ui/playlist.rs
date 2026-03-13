use eframe::egui::{self, Color32, Rect, Pos2};
use crate::bridge::Bridge;
use crate::ui::UiState;
use crate::project::project::Project;
use super::theme;

const TRACK_HEIGHT: f32 = 40.0;
const BAR_WIDTH: f32 = 64.0;
const HEADER_WIDTH: f32 = 120.0;
const NUM_BARS: u32 = 32;

pub fn show(ui: &mut egui::Ui, _bridge: &mut Bridge, ui_state: &mut UiState, project: &mut Project) {
    let num_tracks = project.channels.len();
    let total_width = HEADER_WIDTH + NUM_BARS as f32 * BAR_WIDTH;
    let total_height = num_tracks as f32 * TRACK_HEIGHT;

    egui::ScrollArea::both()
        .id_salt("playlist_scroll")
        .show(ui, |ui| {
            let (response, painter) = ui.allocate_painter(
                egui::vec2(total_width, total_height),
                egui::Sense::click(),
            );
            let origin = response.rect.min;

            // Track headers
            for (i, ch) in project.channels.iter().enumerate() {
                let y = origin.y + i as f32 * TRACK_HEIGHT;
                let color = Color32::from_rgb(ch.color[0], ch.color[1], ch.color[2]);
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(origin.x, y), egui::vec2(HEADER_WIDTH - 2.0, TRACK_HEIGHT - 1.0)),
                    2.0, theme::BG_MID,
                );
                painter.text(
                    Pos2::new(origin.x + 8.0, y + TRACK_HEIGHT / 2.0),
                    egui::Align2::LEFT_CENTER,
                    &ch.name,
                    egui::FontId::proportional(12.0),
                    color,
                );
            }

            // Grid
            let grid_x = origin.x + HEADER_WIDTH;
            for bar in 0..=NUM_BARS {
                let x = grid_x + bar as f32 * BAR_WIDTH;
                let color = if bar % 4 == 0 { Color32::from_rgb(0x44, 0x44, 0x44) } else { Color32::from_rgb(0x2a, 0x2a, 0x2a) };
                painter.line_segment(
                    [Pos2::new(x, origin.y), Pos2::new(x, origin.y + total_height)],
                    egui::Stroke::new(1.0, color),
                );
            }
            for track in 0..=num_tracks {
                let y = origin.y + track as f32 * TRACK_HEIGHT;
                painter.line_segment(
                    [Pos2::new(origin.x, y), Pos2::new(origin.x + total_width, y)],
                    egui::Stroke::new(0.5, Color32::from_rgb(0x2a, 0x2a, 0x2a)),
                );
            }

            // Draw clips
            for track in &project.arrangement.tracks {
                let track_y = origin.y + track.channel_id as f32 * TRACK_HEIGHT;
                let ch = project.channels.get(track.channel_id);
                let clip_color = ch.map(|c| Color32::from_rgba_unmultiplied(c.color[0], c.color[1], c.color[2], 180))
                    .unwrap_or(theme::FL_ORANGE);
                for clip in &track.clips {
                    let x = grid_x + clip.start_bar as f32 * BAR_WIDTH;
                    let w = clip.length_bars as f32 * BAR_WIDTH - 2.0;
                    painter.rect_filled(
                        Rect::from_min_size(Pos2::new(x, track_y + 2.0), egui::vec2(w, TRACK_HEIGHT - 4.0)),
                        3.0, clip_color,
                    );
                }
            }

            // Click to place clip
            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let rel_x = pos.x - grid_x;
                    let rel_y = pos.y - origin.y;
                    if rel_x >= 0.0 && rel_y >= 0.0 {
                        let bar = (rel_x / BAR_WIDTH) as u32;
                        let track_idx = (rel_y / TRACK_HEIGHT) as usize;
                        if track_idx < num_tracks {
                            // Find or create track
                            if let Some(track) = project.arrangement.tracks.iter_mut().find(|t| t.channel_id == track_idx) {
                                track.clips.push(crate::project::arrangement::Clip {
                                    pattern_id: ui_state.selected_pattern,
                                    start_bar: bar,
                                    length_bars: 2,
                                });
                            } else {
                                project.arrangement.tracks.push(crate::project::arrangement::ArrangementTrack {
                                    channel_id: track_idx,
                                    clips: vec![crate::project::arrangement::Clip {
                                        pattern_id: ui_state.selected_pattern,
                                        start_bar: bar,
                                        length_bars: 2,
                                    }],
                                });
                            }
                        }
                    }
                }
            }
        });
}
