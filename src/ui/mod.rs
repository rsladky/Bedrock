pub mod channel_rack;
pub mod mixer;
pub mod piano_roll;
pub mod playlist;
pub mod step_sequencer;
pub mod theme;
pub mod toolbar;
pub mod widgets;

use crate::bridge::shared_state::EngineSnapshot;
use eframe::egui;

#[derive(Debug, Default)]
pub struct UiState {
    pub active_view: ActiveView,
    pub selected_channel: usize,
    pub selected_pattern: usize,
    pub snapshot: EngineSnapshot,
    pub piano_roll_scroll: egui::Vec2,
    pub playlist_scroll: egui::Vec2,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ActiveView {
    #[default]
    StepSequencer,
    PianoRoll,
    Playlist,
}
