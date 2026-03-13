/// Simple Schroeder reverb
pub struct Reverb {
    comb_delays: [[f32; 3556]; 4],
    comb_pos: [usize; 4],
    comb_feedback: f32,
    allpass_delay: [f32; 556],
    allpass_pos: usize,
    pub wet: f32,
    pub room_size: f32,
}

impl Default for Reverb {
    fn default() -> Self {
        Self {
            comb_delays: [[0.0; 3556]; 4],
            comb_pos: [0; 4],
            comb_feedback: 0.7,
            allpass_delay: [0.0; 556],
            allpass_pos: 0,
            wet: 0.3,
            room_size: 0.5,
        }
    }
}

impl Reverb {
    pub fn process(&mut self, input: f32) -> f32 {
        const DELAYS: [usize; 4] = [1557, 1617, 1491, 1422];
        let mut out = 0.0f32;
        for i in 0..4 {
            let delay = DELAYS[i];
            let buf_out = self.comb_delays[i][self.comb_pos[i]];
            let new_val = input + buf_out * self.comb_feedback;
            self.comb_delays[i][self.comb_pos[i]] = new_val;
            self.comb_pos[i] = (self.comb_pos[i] + 1) % delay;
            out += buf_out;
        }
        out *= 0.25;

        // Allpass
        let ap_out = self.allpass_delay[self.allpass_pos];
        let ap_in = out + ap_out * 0.5;
        self.allpass_delay[self.allpass_pos] = ap_in;
        self.allpass_pos = (self.allpass_pos + 1) % 556;
        let result = ap_out - ap_in * 0.5;

        input * (1.0 - self.wet) + result * self.wet
    }
}
