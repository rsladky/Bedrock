use crate::project::arrangement::Arrangement;
use crate::project::channel::Channel;
use crate::project::mixer_state::MixerChannel;
use crate::project::pattern::Pattern;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub format_version: u32,
    pub name: String,
    pub bpm: f64,
    pub time_signature: (u8, u8),
    pub channels: Vec<Channel>,
    pub patterns: Vec<Pattern>,
    pub arrangement: Arrangement,
    pub mixer: Vec<MixerChannel>,
}

impl Default for Project {
    fn default() -> Self {
        let channels = vec![
            Channel::new(0, "Kick", [255, 120, 0]),
            Channel::new(1, "Snare", [0, 180, 255]),
            Channel::new(2, "HiHat", [255, 220, 0]),
            Channel::new(3, "Bass", [0, 255, 120]),
        ];
        let pattern = Pattern::default_with_channels(channels.len());
        Self {
            format_version: 1,
            name: "Untitled".to_string(),
            bpm: 128.0,
            time_signature: (4, 4),
            channels,
            patterns: vec![pattern],
            arrangement: Arrangement::default(),
            mixer: (0..4)
                .map(|i| {
                    let name = if i == 0 {
                        "Master".to_string()
                    } else {
                        format!("Ch{}", i)
                    };
                    MixerChannel::new(i, &name)
                })
                .collect(),
        }
    }
}

impl Project {
    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
}
