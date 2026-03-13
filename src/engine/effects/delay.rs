pub struct Delay {
    buffer: Vec<f32>,
    pos: usize,
    pub feedback: f32,
    pub wet: f32,
    delay_samples: usize,
}

impl Delay {
    pub fn new(sample_rate: f32, delay_ms: f32) -> Self {
        let delay_samples = (sample_rate * delay_ms / 1000.0) as usize;
        let capacity = delay_samples.max(1);
        Self {
            buffer: vec![0.0; capacity],
            pos: 0,
            feedback: 0.4,
            wet: 0.3,
            delay_samples: capacity,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let delayed = self.buffer[self.pos];
        self.buffer[self.pos] = input + delayed * self.feedback;
        self.pos = (self.pos + 1) % self.delay_samples;
        input * (1.0 - self.wet) + delayed * self.wet
    }
}
