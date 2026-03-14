pub struct Sampler {
    pub sample_data: Vec<f32>,
    pub sample_rate: f32,
    pub playback_rate: f32,
    pub position: f64,
    pub playing: bool,
}

impl Sampler {
    pub fn new(sample_data: Vec<f32>, sample_rate: f32) -> Self {
        Self {
            sample_data,
            sample_rate,
            playback_rate: 1.0,
            position: 0.0,
            playing: false,
        }
    }

    pub fn trigger(&mut self) {
        self.position = 0.0;
        self.playing = true;
    }

    pub fn render_sample(&mut self) -> f32 {
        if !self.playing || self.sample_data.is_empty() {
            return 0.0;
        }
        let idx = self.position as usize;
        if idx >= self.sample_data.len() {
            self.playing = false;
            return 0.0;
        }
        // Hermite interpolation
        let frac = self.position - idx as f64;
        let s0 = self
            .sample_data
            .get(idx.saturating_sub(1))
            .copied()
            .unwrap_or(0.0);
        let s1 = self.sample_data.get(idx).copied().unwrap_or(0.0);
        let s2 = self.sample_data.get(idx + 1).copied().unwrap_or(0.0);
        let s3 = self.sample_data.get(idx + 2).copied().unwrap_or(0.0);

        let c0 = s1;
        let c1 = (s2 - s0) * 0.5;
        let c2 = s0 - 2.5 * s1 + 2.0 * s2 - 0.5 * s3;
        let c3 = (-0.5 * s0) + 1.5 * s1 - 1.5 * s2 + 0.5 * s3;
        let t = frac as f32;
        let out = ((c3 * t + c2) * t + c1) * t + c0;

        self.position += self.playback_rate as f64;
        out
    }
}
