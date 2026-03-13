# Bedrock — TODO

Phases follow the implementation plan. Current status: **Phase 1–6 scaffolded**, no phase fully polished.

---

## Phase 1 — Audio Engine ✅ (scaffolded)
- [x] cpal stream setup with runtime sample rate detection
- [x] 8-voice ADSR sine synth (`VoicePool`)
- [x] Lock-free bridge: rtrb commands + events, Arc<Mutex> snapshot
- [ ] Verify < 5% CPU in `top` with sine tone playing
- [ ] Confirm no xrun log messages on Apple Silicon

## Phase 2 — Project Model ✅ (scaffolded)
- [x] `Project`, `Channel`, `Pattern`, `StepGrid`, `PianoRollData`, `Note`
- [x] `Arrangement`, `MixerChannel`, `EffectSlot`
- [x] `project.save()` / `Project::load()` (serde_json)
- [ ] Add `format_version` migration logic for future schema changes
- [ ] Write basic round-trip test (save → load → compare)

## Phase 3 — egui Shell ✅ (scaffolded)
- [x] Dark FL-style theme (`#1e1e1e` / `#ff7800`)
- [x] 4-panel layout: toolbar, channel rack, central panel, mixer
- [x] Tab switching: Step Seq / Piano Roll / Playlist
- [ ] Load custom font (JetBrainsMono) — `assets/fonts/` not yet wired up
- [ ] File menu: New / Open / Save / Save As

## Phase 4 — Step Sequencer ✅ (scaffolded)
- [x] 16-step grid renders with channel color coding
- [x] Click to toggle steps, sends `SetPattern` to engine
- [x] Active step highlighted in FL orange during playback
- [ ] Beat grouping visual (every 4 steps slightly brighter bg)
- [ ] Per-step velocity (right-click → drag to set)
- [ ] Pattern selector (multiple patterns, not just index 0)
- [ ] Mute/solo per channel row

## Phase 5 — Piano Roll ✅ (scaffolded)
- [x] Scrollable grid with 8-octave key sidebar
- [x] Black/white key coloring
- [x] Beat + bar grid lines
- [x] Click to add notes (snapped to beat)
- [ ] Note resize (drag right edge)
- [ ] Note delete (right-click)
- [ ] Note drag/move
- [ ] Snap options: 1/4, 1/8, 1/16, 1/32
- [ ] Piano roll playhead following `EngineSnapshot.playhead_sample`
- [ ] Audition note on click (send `NoteOn`/`NoteOff`)

## Phase 6 — Playlist / Arrangement ✅ (scaffolded)
- [x] Track headers with channel colors
- [x] Bar grid (32 bars)
- [x] Click to place clips
- [ ] Clip delete (right-click)
- [ ] Clip drag/move
- [ ] Clip resize
- [ ] Arrangement playback (engine reads clips, not just step sequencer)
- [ ] Loop region marker

## Phase 7 — Mixer
- [x] Fader + pan + mute per channel (UI only)
- [ ] Wire `SetMixerVolume` / `SetMuted` through to audio engine
- [ ] VU meters reading from `EngineSnapshot.peak_levels`
- [ ] Per-channel effect slots (Reverb, Delay, EQ) — structs exist, not wired
- [ ] Effect parameter UI (knobs for wet/dry, room size, etc.)
- [ ] Send/return routing

## Phase 8 — MIDI Input
- [ ] Add `midir` to Cargo.toml
- [ ] `MidiManager::new()` — open first available MIDI input
- [ ] Route MIDI bytes → `EngineCommand` via `midi/router.rs`
- [ ] MIDI learn for BPM, volume, etc.
- [ ] MIDI channel → Bedrock channel mapping

## Phase 9 — Sampler Instrument
- [ ] Add `symphonia` + `hound` to Cargo.toml
- [ ] Load WAV/MP3 into `engine/sampler.rs`
- [ ] `SamplerConfig.file_path` wired to actual file loading
- [ ] File picker in channel rack for sampler channels
- [ ] Pitch shifting via playback rate

## Phase 10 — Polish
- [ ] Export to WAV (`hound` render loop)
- [ ] Undo/redo stack (simple command history on `Project`)
- [ ] CPU load measurement (time `fill_buffer` duration vs budget)
- [ ] Panic handler that stops audio stream cleanly
- [ ] App icon

## Keyboard Shortcuts
- [ ] `Space` — Play / Stop toggle
- [ ] `Ctrl+Z` / `Ctrl+Shift+Z` — Undo / Redo
- [ ] `Ctrl+S` — Save project
- [ ] `Ctrl+Shift+S` — Save As
- [ ] `Ctrl+N` — New project
- [ ] `Ctrl+O` — Open project
- [ ] `F5` — Switch to Step Sequencer
- [ ] `F6` — Switch to Piano Roll
- [ ] `F7` — Switch to Playlist
- [ ] `F9` — Switch to Mixer (focus bottom panel)
- [ ] `Delete` — Delete selected note / clip
- [ ] `Ctrl+A` — Select all notes (piano roll) / clips (playlist)
- [ ] `Ctrl+D` — Duplicate selected pattern / clip
- [ ] `+` / `-` — Increase / decrease BPM by 1
- [ ] `Ctrl++` / `Ctrl+-` — Zoom in / out (piano roll & playlist)
- [ ] `1`–`9` — Select channel 1–9 in channel rack
- [ ] `M` — Mute selected channel
- [ ] `Escape` — Deselect / cancel current action

---

## Known Issues / Tech Debt
- `fill_buffer` writes to `Arc<Mutex<EngineSnapshot>>` every 512 frames — fine for now, but replace with `triple_buffer` if contention appears under load
- Step sequencer triggers all active-step notes at pitch `60 + ch * 5` — should use per-channel instrument config
- `VoicePool::note_on` steals voice at index 0 on overflow — should steal by oldest or lowest velocity
- Piano roll note snap only snaps to beat (96 ticks) — need configurable snap grid
- No error recovery if cpal device is unplugged at runtime
