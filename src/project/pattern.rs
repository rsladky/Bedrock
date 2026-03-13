use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: usize,
    pub name: String,
    pub step_grid: StepGrid,
    pub piano_roll: PianoRollData,
}

impl Pattern {
    pub fn default_with_channels(num_channels: usize) -> Self {
        Self {
            id: 0,
            name: "Pattern 1".to_string(),
            step_grid: StepGrid::new(num_channels, 16),
            piano_roll: PianoRollData::default(),
        }
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self::default_with_channels(4)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepGrid {
    pub steps: u8,
    pub cells: Vec<Vec<u8>>, // [channel][step] = velocity (0 = off)
}

impl StepGrid {
    pub fn new(num_channels: usize, steps: u8) -> Self {
        Self {
            steps,
            cells: vec![vec![0u8; steps as usize]; num_channels],
        }
    }

    pub fn toggle(&mut self, channel: usize, step: usize) {
        if let Some(row) = self.cells.get_mut(channel) {
            if let Some(cell) = row.get_mut(step) {
                *cell = if *cell == 0 { 100 } else { 0 };
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PianoRollData {
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub channel_id: usize,
    pub pitch: u8,
    pub velocity: u8,
    pub start_tick: u32,
    pub duration_ticks: u32,
}
