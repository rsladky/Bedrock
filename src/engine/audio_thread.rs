use crate::bridge::EngineHandle;
use crate::bridge::commands::EngineCommand;
use crate::bridge::events::EngineEvent;
use crate::bridge::shared_state::EngineSnapshot;
use crate::engine::synth::VoicePool;
use crate::engine::sequencer::StepSequencer;

pub struct AudioState {
    sample_rate: f64,
    bpm: f64,
    channels: usize,
    handle: EngineHandle,
    playing: bool,
    playhead_sample: u64,
    voices: VoicePool,
    sequencer: StepSequencer,
    snapshot_counter: u32,
    peak_left: f32,
    peak_right: f32,
}

impl AudioState {
    pub fn new(sample_rate: f64, bpm: f64, channels: usize, handle: EngineHandle) -> Self {
        Self {
            sample_rate,
            bpm,
            channels,
            handle,
            playing: false,
            playhead_sample: 0,
            voices: VoicePool::new(),
            sequencer: StepSequencer::new(16),
            snapshot_counter: 0,
            peak_left: 0.0,
            peak_right: 0.0,
        }
    }

    pub fn fill_buffer(&mut self, data: &mut [f32]) {
        // Drain commands
        while let Ok(cmd) = self.handle.cmd_rx.pop() {
            self.process_command(cmd);
        }

        let samples_per_step = (self.sample_rate * 60.0 / self.bpm / 4.0) as u64;

        let frame_count = data.len() / self.channels;
        for frame_idx in 0..frame_count {
            if self.playing {
                if self.sequencer.tick(self.playhead_sample, samples_per_step) {
                    let step = self.sequencer.current_step;
                    for ch in 0..self.sequencer.num_channels() {
                        if self.sequencer.is_step_active(ch, step) {
                            self.voices.note_on(ch, 60 + ch as u8 * 5, 100);
                        }
                    }
                    let _ = self.handle.event_tx.push(EngineEvent::StepAdvanced { step });
                }
                self.playhead_sample += 1;
            }

            let (l, r) = self.voices.render(self.sample_rate);
            let base = frame_idx * self.channels;
            if self.channels >= 2 {
                data[base] = l;
                data[base + 1] = r;
            } else if self.channels == 1 {
                data[base] = (l + r) * 0.5;
            }

            self.peak_left = self.peak_left.max(l.abs());
            self.peak_right = self.peak_right.max(r.abs());
        }

        self.snapshot_counter += frame_count as u32;
        if self.snapshot_counter >= 512 {
            self.snapshot_counter = 0;
            let snap = EngineSnapshot {
                playhead_sample: self.playhead_sample,
                playing: self.playing,
                peak_levels: vec![self.peak_left, self.peak_right],
                cpu_load: 0.0,
                active_step: self.sequencer.current_step,
            };
            self.handle.write_snapshot(snap);
            self.peak_left *= 0.9;
            self.peak_right *= 0.9;
        }
    }

    fn process_command(&mut self, cmd: EngineCommand) {
        match cmd {
            EngineCommand::Play => {
                self.playing = true;
                self.playhead_sample = 0;
                self.sequencer.reset();
            }
            EngineCommand::Stop => {
                self.playing = false;
                self.voices.all_notes_off();
            }
            EngineCommand::SetBpm(bpm) => self.bpm = bpm,
            EngineCommand::NoteOn { channel_id, pitch, velocity } => {
                self.voices.note_on(channel_id, pitch, velocity);
            }
            EngineCommand::NoteOff { channel_id, pitch: _ } => {
                self.voices.note_off(channel_id);
            }
            EngineCommand::SetPattern(pattern) => {
                self.sequencer.set_pattern(*pattern);
            }
            EngineCommand::SetVolume { .. } => {}
            EngineCommand::SetPan { .. } => {}
            EngineCommand::SetMuted { .. } => {}
            EngineCommand::SetMixerVolume { .. } => {}
        }
    }
}
