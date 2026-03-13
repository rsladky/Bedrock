use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Arrangement {
    pub tracks: Vec<ArrangementTrack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrangementTrack {
    pub channel_id: usize,
    pub clips: Vec<Clip>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub pattern_id: usize,
    pub start_bar: u32,
    pub length_bars: u32,
}
