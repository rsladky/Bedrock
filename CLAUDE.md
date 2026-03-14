# Bedrock — CLAUDE.md

FL Studio clone in Rust (macOS). Learning project for audio programming, DSP, and Rust GUI dev.

## Build & Run

```bash
cargo run          # dev build (opt-level=1 to avoid audio glitches)
cargo build --release
cargo check        # fast type-check without linking
```

## Stack

| Layer | Crate | Version |
|---|---|---|
| GUI | egui + eframe | 0.29 |
| Audio I/O | cpal | 0.15 |
| Audio ↔ GUI | rtrb (ring buffer) | 0.3 |
| Shared state | Arc<parking_lot::Mutex> | — |
| Serialization | serde + serde_json | 1.0 |

## Architecture

```
GUI Thread                          Audio Thread (cpal callback)
──────────────────────────────────────────────────────────────
rtrb::Producer<EngineCommand>  ──►  Consumer drains in fill_buffer()
rtrb::Consumer<EngineEvent>    ◄──  Producer writes StepAdvanced, Xrun
Arc<Mutex<EngineSnapshot>>     ◄──  Written every ~512 samples
```

**Golden rule**: `fill_buffer()` in `engine/audio_thread.rs` is the hot path. Never allocate heap memory, block on a mutex, or do I/O inside it. The `Arc<Mutex<EngineSnapshot>>` write happens only every 512 frames — acceptable latency.

## Key Files

| File | Role |
|---|---|
| `src/engine/audio_thread.rs` | cpal callback — THE HOT PATH |
| `src/engine/synth.rs` | 8-voice ADSR sine synth, `VoicePool` |
| `src/engine/sequencer.rs` | Step sequencer tick logic |
| `src/bridge/mod.rs` | Owns rtrb pairs + snapshot Arc |
| `src/bridge/commands.rs` | `EngineCommand` enum (GUI → audio) |
| `src/project/pattern.rs` | `StepGrid`, `PianoRollData`, `Note` |
| `src/app.rs` | Top-level `eframe::App` |
| `src/ui/theme.rs` | Dark palette constants |

## Data Model

```
Project
  ├── channels: Vec<Channel>        (instrument config, volume, pan)
  ├── patterns: Vec<Pattern>
  │     ├── step_grid: StepGrid     ([channel][step] = velocity)
  │     └── piano_roll: PianoRollData (Vec<Note>)
  ├── arrangement: Arrangement      (Vec<ArrangementTrack> with Clips)
  └── mixer: Vec<MixerChannel>
```

Project is serializable to JSON via serde. Use `project.save(path)` / `Project::load(path)`.

## Timing

```
samples_per_beat = sample_rate * 60.0 / bpm
samples_per_step = samples_per_beat / 4   (16 steps at 4/4)
PPQ = 96 ticks per beat (piano roll)
```

Always use `playhead_sample` as ground truth — never wall-clock time.

## UI Layout

```
┌──────────────────────────────────────────────────────┐
│  TOOLBAR  [▶][■]  BPM: 128  4/4  CPU: 3%  [meters] │
├──────────────┬───────────────────────────────────────┤
│ CHANNEL RACK │  [Step Seq] [Piano Roll] [Playlist]   │
│ Kick         │  (central panel)                      │
│ Snare        │                                       │
│ HiHat        │                                       │
│ Bass         │                                       │
├──────────────┴───────────────────────────────────────┤
│  MIXER  [Master][Ch1][Ch2]...  faders + pan + mute  │
└──────────────────────────────────────────────────────┘
```

Theme constants are in `src/ui/theme.rs`: `BG_DARK` (`#1e1e1e`), `FL_ORANGE` (`#ff7800`).

## macOS Notes

- Apple Silicon defaults to **48000 Hz** — always query `device.default_output_config()`, never hardcode sample rate.
- Audio permission prompt appears on first run (microphone/output entitlement).

## Code Conventions

- **Naming**: snake_case for files/functions, PascalCase for types/enums. Enum variants are PascalCase with no prefix (e.g. `Waveform::Sine`, not `WaveformSine`).
- **Derive order**: `Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize` (Copy only for small value types).
- **Module structure**: one `mod.rs` per directory re-exports public items. Leaf files are named for the primary type they define (e.g. `synth.rs` → `VoicePool`, `Waveform`).
- **Pub fields**: data structs use `pub` fields directly (no getters) — keeps things simple for a learning project.
- **UI widgets**: custom widgets live in `src/ui/widgets/` as separate files, one per widget.
- **Constants**: theme colors are `pub const` in `src/ui/theme.rs`. Use these instead of inline hex values.
- **Formatting**: `.rustfmt.toml` at repo root — run `cargo fmt` before committing.
- **Lints**: `cargo clippy -- -D warnings` must pass.

## What to Avoid

- **Heap allocation in `fill_buffer()`**: no `Vec::push`, `String`, `Box::new`, `format!`, or `println!` on the audio thread. Use pre-allocated buffers.
- **Blocking in the audio callback**: no `Mutex::lock()` (use `try_lock`), no file I/O, no network. The only exception is the snapshot write every 512 frames which uses `parking_lot::Mutex::try_lock`.
- **Hardcoded sample rates**: always derive from `cpal::StreamConfig`. 44100 is wrong on Apple Silicon (48000).
- **Wall-clock time for audio timing**: use `playhead_sample` as ground truth, never `Instant::now()` or `SystemTime`.
- **Large messages over rtrb**: the ring buffer is fixed-size. Use `Box<T>` for large payloads (see `SetPattern(Box<Pattern>)`).
- **Unwrap in production paths**: use `anyhow::Result` or handle errors. `unwrap()` is acceptable only in tests or one-time init code.

## Adding a New EngineCommand

1. Add variant to `src/bridge/commands.rs`
2. Handle it in `AudioState::process_command()` in `src/engine/audio_thread.rs`
3. Send it via `bridge.send(EngineCommand::...)` from the GUI thread
