pub struct MixerChannel {
    pub volume: f32,
    pub pan: f32,
    pub muted: bool,
}

impl Default for MixerChannel {
    fn default() -> Self {
        Self {
            volume: 1.0,
            pan: 0.0,
            muted: false,
        }
    }
}

pub struct Mixer {
    pub channels: Vec<MixerChannel>,
}

impl Mixer {
    pub fn new(num_channels: usize) -> Self {
        Self {
            channels: (0..num_channels).map(|_| MixerChannel::default()).collect(),
        }
    }

    pub fn process(&self, channel_idx: usize, l: f32, r: f32) -> (f32, f32) {
        let ch = &self.channels[channel_idx.min(self.channels.len() - 1)];
        if ch.muted {
            return (0.0, 0.0);
        }
        let pan_l = if ch.pan > 0.0 { 1.0 - ch.pan } else { 1.0 };
        let pan_r = if ch.pan < 0.0 { 1.0 + ch.pan } else { 1.0 };
        (l * ch.volume * pan_l, r * ch.volume * pan_r)
    }
}
