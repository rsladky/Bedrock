#[derive(Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Waveform {
    #[default]
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SynthConfig {
    pub waveform: Waveform,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub cutoff: f32,
    pub resonance: f32,
}

#[derive(Clone, Debug, Default)]
pub struct AdsrEnvelope {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    phase: EnvPhase,
    level: f32,
    time: f32,
}

#[derive(Clone, Debug, Default, PartialEq)]
enum EnvPhase {
    #[default]
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

impl AdsrEnvelope {
    pub fn note_on(&mut self) {
        self.phase = EnvPhase::Attack;
        self.time = 0.0;
    }
    pub fn note_off(&mut self) {
        if self.phase != EnvPhase::Idle {
            self.phase = EnvPhase::Release;
            self.time = 0.0;
        }
    }
    pub fn tick(&mut self, dt: f32) -> f32 {
        match self.phase {
            EnvPhase::Idle => 0.0,
            EnvPhase::Attack => {
                self.time += dt;
                let a = self.attack.max(0.001);
                self.level = (self.time / a).min(1.0);
                if self.time >= a { self.phase = EnvPhase::Decay; self.time = 0.0; }
                self.level
            }
            EnvPhase::Decay => {
                self.time += dt;
                let d = self.decay.max(0.001);
                self.level = 1.0 - (1.0 - self.sustain) * (self.time / d).min(1.0);
                if self.time >= d { self.phase = EnvPhase::Sustain; }
                self.level
            }
            EnvPhase::Sustain => self.sustain,
            EnvPhase::Release => {
                self.time += dt;
                let r = self.release.max(0.001);
                self.level = self.sustain * (1.0 - (self.time / r).min(1.0));
                if self.time >= r { self.phase = EnvPhase::Idle; self.level = 0.0; }
                self.level
            }
        }
    }
    pub fn is_idle(&self) -> bool { self.phase == EnvPhase::Idle }
}

#[derive(Clone, Debug, Default)]
pub struct Voice {
    pub active: bool,
    pub channel_id: usize,
    pub pitch: u8,
    pub phase: f64,
    pub env: AdsrEnvelope,
    pub velocity: f32,
}

impl Voice {
    fn frequency(pitch: u8) -> f64 {
        440.0 * 2.0f64.powf((pitch as f64 - 69.0) / 12.0)
    }

    pub fn render_sample(&mut self, sample_rate: f64) -> f32 {
        if !self.active { return 0.0; }
        let dt = 1.0 / sample_rate as f32;
        let env = self.env.tick(dt);
        if self.env.is_idle() { self.active = false; return 0.0; }
        let freq = Self::frequency(self.pitch);
        let out = (self.phase * std::f64::consts::TAU).sin() as f32;
        self.phase += freq / sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; }
        out * env * self.velocity
    }
}

pub struct VoicePool {
    voices: [Voice; 8],
}

impl VoicePool {
    pub fn new() -> Self {
        Self { voices: std::array::from_fn(|_| Voice::default()) }
    }

    pub fn note_on(&mut self, channel_id: usize, pitch: u8, velocity: u8) {
        // Find idle or steal oldest
        let slot = self.voices.iter().position(|v| !v.active)
            .unwrap_or(0);
        let v = &mut self.voices[slot];
        v.active = true;
        v.channel_id = channel_id;
        v.pitch = pitch;
        v.phase = 0.0;
        v.velocity = velocity as f32 / 127.0;
        v.env = AdsrEnvelope {
            attack: 0.01,
            decay: 0.1,
            sustain: 0.7,
            release: 0.3,
            ..Default::default()
        };
        v.env.note_on();
    }

    pub fn note_off(&mut self, channel_id: usize) {
        for v in &mut self.voices {
            if v.active && v.channel_id == channel_id {
                v.env.note_off();
            }
        }
    }

    pub fn all_notes_off(&mut self) {
        for v in &mut self.voices {
            v.active = false;
        }
    }

    pub fn render(&mut self, sample_rate: f64) -> (f32, f32) {
        let mut mix = 0.0f32;
        for v in &mut self.voices {
            mix += v.render_sample(sample_rate);
        }
        let out = mix * 0.25; // Reduce gain to avoid clipping
        (out, out)
    }
}
