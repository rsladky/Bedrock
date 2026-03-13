#[derive(Debug, Clone, Default)]
pub struct EngineSnapshot {
    pub playhead_sample: u64,
    pub playing: bool,
    pub peak_levels: Vec<f32>,
    pub cpu_load: f32,
    pub active_step: usize,
}
