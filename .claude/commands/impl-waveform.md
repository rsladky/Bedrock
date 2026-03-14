Implement the `$ARGUMENTS` waveform in `src/engine/synth.rs`, wire it to the engine command pipeline, and expose it from the GUI.

`$ARGUMENTS` must be one of: `Square`, `Sawtooth`, `Triangle`.

## Step 1 — Read the current synth

Read `src/engine/synth.rs` in full before touching anything.

## Step 2 — Extend the `Waveform` enum

If a `Waveform` enum doesn't exist yet, create it above the `Voice` struct:

```rust
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub enum Waveform {
    #[default]
    Sine,
    Square,
    Sawtooth,
    Triangle,
}
```

If it exists, add the missing variant.

## Step 3 — Add `waveform` field to `Voice`

```rust
pub struct Voice {
    // … existing fields …
    pub waveform: Waveform,
}
```

Initialise it in `Voice::new()` / `VoicePool::new()` as `Waveform::Sine` (keeps existing behaviour).

## Step 4 — Implement DSP in `render_sample()`

`self.phase` must stay in `[0.0, 1.0)` and be advanced **after** sampling.

```rust
// Square
let sample = if self.phase < 0.5 { 1.0_f32 } else { -1.0_f32 };

// Sawtooth
let sample = 2.0_f32 * self.phase - 1.0_f32;

// Triangle
let sample = if self.phase < 0.5 {
    4.0_f32 * self.phase - 1.0_f32
} else {
    3.0_f32 - 4.0_f32 * self.phase
};
```

Add the arm for `$ARGUMENTS` inside the existing `match self.waveform { … }`. If no match exists, convert the current sine expression into `Waveform::Sine =>` first.

**Hot-path constraints:**
- No allocation, no lookup table created at runtime, no mutex lock
- Pure arithmetic only — these formulas are O(1) per sample

## Step 5 — Wire via `SetSynthConfig` command

Follow the `/new-engine-command` workflow to add:

```rust
// src/bridge/commands.rs
SetWaveform { channel: usize, waveform: synth::Waveform },
```

Handle it in `process_command()` by setting the waveform on all active voices for that channel (or on `VoicePool::default_waveform` if you prefer a pool-level field).

## Step 6 — GUI selector

In the appropriate panel (channel rack or instrument panel), add an `egui::ComboBox` with the four waveform variants. Send `EngineCommand::SetWaveform` on `.changed()`.

## Verification

- [ ] Existing `Sine` behaviour unchanged (same formula in its arm)
- [ ] `cargo check` clean
- [ ] Audition the waveform: `cargo run`, trigger a note, hear the expected shape
- [ ] No allocations inside `render_sample()` or `process_command()`
