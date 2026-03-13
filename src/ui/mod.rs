pub mod theme;
pub mod toolbar;
pub mod step_sequencer;
pub mod piano_roll;
pub mod playlist;
pub mod mixer;
pub mod channel_rack;
pub mod widgets;

use eframe::egui;
use crate::bridge::shared_state::EngineSnapshot;

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
