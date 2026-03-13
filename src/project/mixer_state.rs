use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixerChannel {
    pub id: usize,
    pub name: String,
    pub volume: f32,
    pub pan: f32,
    pub muted: bool,
    pub effects: Vec<EffectSlot>,
}

impl MixerChannel {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            volume: 1.0,
            pan: 0.0,
            muted: false,
            effects: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectSlot {
    pub effect_type: EffectType,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Reverb,
    Delay,
    Eq,
}
