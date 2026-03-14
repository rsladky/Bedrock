# Bedrock Roadmap

FL Studio clone in Rust — learning project for audio programming, DSP, and Rust GUI dev.

## Phase 1: Core Audio Pipeline

Wire up the existing engine components so sound flows end-to-end through the mixer.

- [ ] Mixer volume/pan actually applied in `fill_buffer()` per-channel
- [ ] Waveform selection (square, saw, triangle) wired through EngineCommand
- [ ] Effect chain integration (reverb, delay, EQ stubs exist — connect to mixer channels)
- [ ] Master channel summing with peak metering
- [ ] Step sequencer triggers synth voices correctly for all channels

## Phase 2: MIDI & Input

- [ ] MIDI device enumeration and input (midir or cpal MIDI)
- [ ] Piano roll note playback through the engine
- [ ] Computer keyboard as MIDI input (Z-M row = C3-B3)
- [ ] MIDI learn for knobs/faders
- [ ] Velocity-sensitive note rendering in piano roll

## Phase 3: Sampler & Sound Design

- [ ] WAV/AIFF sample loading (hound or symphonia)
- [ ] Sample playback engine (pitch-shift via resampling)
- [ ] Waveform display widget for loaded samples
- [ ] Per-channel instrument type: Synth or Sampler
- [ ] Basic filter (low-pass, high-pass) on synth voices

## Phase 4: Arrangement & Automation

- [ ] Playlist/arrangement view with clip placement
- [ ] Pattern-to-clip workflow (place patterns on arrangement tracks)
- [ ] Parameter automation lanes (volume, pan, filter cutoff)
- [ ] Audio recording from cpal input device
- [ ] Song-mode playback (arrangement top-to-bottom)

## Phase 5: Polish & Export

- [ ] Audio export to WAV (offline render)
- [ ] Undo/redo system (command pattern)
- [ ] CPU load reporting (measure `fill_buffer()` duration vs budget)
- [ ] Native file dialogs (rfd crate)
- [ ] Project autosave
- [ ] Keyboard shortcuts for transport (space = play/stop, etc.)
