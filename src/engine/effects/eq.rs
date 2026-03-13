/// 3-band biquad EQ
pub struct Eq {
    pub low_gain: f32,
    pub mid_gain: f32,
    pub high_gain: f32,
    low: BiquadFilter,
    high: BiquadFilter,
}

impl Default for Eq {
    fn default() -> Self {
        let mut low = BiquadFilter::default();
        low.set_lowshelf(200.0, 48000.0, 0.0);
        let mut high = BiquadFilter::default();
        high.set_highshelf(5000.0, 48000.0, 0.0);
        Self { low_gain: 0.0, mid_gain: 0.0, high_gain: 0.0, low, high }
    }
}

impl Eq {
    pub fn process(&mut self, input: f32) -> f32 {
        let s = self.low.process(input);
        self.high.process(s)
    }
}

#[derive(Default, Clone)]
pub struct BiquadFilter {
    b0: f32, b1: f32, b2: f32,
    a1: f32, a2: f32,
    x1: f32, x2: f32,
    y1: f32, y2: f32,
}

impl BiquadFilter {
    pub fn set_lowshelf(&mut self, freq: f32, sample_rate: f32, gain_db: f32) {
        let a = 10.0f32.powf(gain_db / 40.0);
        let w0 = std::f32::consts::TAU * freq / sample_rate;
        let (s, c) = (w0.sin(), w0.cos());
        let beta = s * (a + 1.0 / a).sqrt();
        self.b0 = a * ((a + 1.0) - (a - 1.0) * c + beta);
        self.b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * c);
        self.b2 = a * ((a + 1.0) - (a - 1.0) * c - beta);
        let a0 = (a + 1.0) + (a - 1.0) * c + beta;
        self.a1 = -2.0 * ((a - 1.0) + (a + 1.0) * c);
        self.a2 = (a + 1.0) + (a - 1.0) * c - beta;
        // Normalize
        self.b0 /= a0; self.b1 /= a0; self.b2 /= a0;
        self.a1 /= a0; self.a2 /= a0;
    }

    pub fn set_highshelf(&mut self, freq: f32, sample_rate: f32, gain_db: f32) {
        let a = 10.0f32.powf(gain_db / 40.0);
        let w0 = std::f32::consts::TAU * freq / sample_rate;
        let (s, c) = (w0.sin(), w0.cos());
        let beta = s * (a + 1.0 / a).sqrt();
        self.b0 = a * ((a + 1.0) + (a - 1.0) * c + beta);
        self.b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * c);
        self.b2 = a * ((a + 1.0) + (a - 1.0) * c - beta);
        let a0 = (a + 1.0) - (a - 1.0) * c + beta;
        self.a1 = 2.0 * ((a - 1.0) - (a + 1.0) * c);
        self.a2 = (a + 1.0) - (a - 1.0) * c - beta;
        self.b0 /= a0; self.b1 /= a0; self.b2 /= a0;
        self.a1 /= a0; self.a2 /= a0;
    }

    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2
              - self.a1 * self.y1 - self.a2 * self.y2;
        self.x2 = self.x1; self.x1 = x;
        self.y2 = self.y1; self.y1 = y;
        y
    }
}
