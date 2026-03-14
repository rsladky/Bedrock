#[derive(Debug, Clone)]
pub enum EngineCommand {
    Play,
    Stop,
    SetBpm(f64),
    NoteOn {
        channel_id: usize,
        pitch: u8,
        velocity: u8,
    },
    NoteOff {
        channel_id: usize,
        pitch: u8,
    },
    SetPattern(Box<crate::project::pattern::Pattern>),
    SetVolume {
        channel_id: usize,
        volume: f32,
    },
    SetPan {
        channel_id: usize,
        pan: f32,
    },
    SetMuted {
        channel_id: usize,
        muted: bool,
    },
    SetMixerVolume {
        channel_idx: usize,
        volume: f32,
    },
}
