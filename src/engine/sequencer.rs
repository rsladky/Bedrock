use crate::project::pattern::Pattern;

pub struct StepSequencer {
    pub current_step: usize,
    pub num_steps: usize,
    last_step_sample: u64,
    pattern: Option<Pattern>,
}

impl StepSequencer {
    pub fn new(num_steps: usize) -> Self {
        Self {
            current_step: 0,
            num_steps,
            last_step_sample: 0,
            pattern: None,
        }
    }

    pub fn reset(&mut self) {
        self.current_step = 0;
        self.last_step_sample = 0;
    }

    /// Returns true if a new step was triggered
    pub fn tick(&mut self, playhead: u64, samples_per_step: u64) -> bool {
        if playhead - self.last_step_sample >= samples_per_step {
            self.last_step_sample = playhead;
            self.current_step = (self.current_step + 1) % self.num_steps;
            true
        } else {
            false
        }
    }

    pub fn num_channels(&self) -> usize {
        self.pattern
            .as_ref()
            .map(|p| p.step_grid.cells.len())
            .unwrap_or(0)
    }

    pub fn is_step_active(&self, channel: usize, step: usize) -> bool {
        self.pattern
            .as_ref()
            .and_then(|p| p.step_grid.cells.get(channel))
            .and_then(|row| row.get(step))
            .map(|&v| v > 0)
            .unwrap_or(false)
    }

    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.num_steps = pattern.step_grid.steps as usize;
        self.pattern = Some(pattern);
    }
}
