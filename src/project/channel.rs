use crate::engine::synth::SynthConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: usize,
    pub name: String,
    pub instrument: InstrumentConfig,
    pub volume: f32,
    pub pan: f32,
    pub muted: bool,
    pub soloed: bool,
    pub color: [u8; 3],
}

impl Channel {
    pub fn new(id: usize, name: &str, color: [u8; 3]) -> Self {
        Self {
            id,
            name: name.to_string(),
            instrument: InstrumentConfig::Synth(SynthConfig::default()),
            volume: 1.0,
            pan: 0.0,
            muted: false,
            soloed: false,
            color,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstrumentConfig {
    Synth(SynthConfig),
    Sampler(SamplerConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SamplerConfig {
    pub file_path: String,
    pub base_pitch: u8,
}
