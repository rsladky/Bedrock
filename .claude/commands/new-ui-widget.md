Scaffold an egui widget and wire it to the audio engine. Description: `$ARGUMENTS`

## Step 1 — Pick the right panel

Read `src/app.rs` to understand the top-level layout, then read the relevant panel file:

| Where it belongs | File |
|---|---|
| Transport / global controls | `src/ui/toolbar.rs` |
| Per-channel instrument settings | `src/ui/channel_rack.rs` |
| Step sequencer grid | `src/ui/step_sequencer.rs` |
| Piano roll notes | `src/ui/piano_roll.rs` |
| Faders, pan, mute/solo | `src/ui/mixer.rs` |

If the file doesn't exist yet, create it and register it in `src/ui/mod.rs`.

## Step 2 — Theme rules (mandatory)

Always import and use palette constants — never hardcode colours:

```rust
use crate::ui::theme;

// correct
stroke: egui::Stroke::new(1.0, theme::FL_ORANGE),
fill: theme::BG_DARK,

// forbidden
stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 120, 0)), // ← don't do this
```

Key constants in `src/ui/theme.rs`: `BG_DARK`, `BG_MID`, `BG_LIGHT`, `FL_ORANGE`, `TEXT_PRIMARY`, `TEXT_DIM`.

## Step 3 — Widget implementation

Use the egui response pattern:

```rust
let response = ui.add(egui::Slider::new(&mut value, 0.0..=1.0).text("Label"));
// or
let clicked = ui.button("Label").clicked();
```

Gate all engine sends on the response:

```rust
if response.changed() {
    // update project first (serialisation ground truth)
    self.project.channels[idx].volume = value;
    // then send to audio engine
    let _ = self.bridge.send(EngineCommand::SetVolume { channel: idx, value });
}
```

**Never** call `bridge.send()` unconditionally every frame — egui redraws at ~60 fps and the rtrb ring buffer will fill up.

## Step 4 — Use `$ARGUMENTS` to decide payload

Based on the description, choose or create the appropriate `EngineCommand` variant. If a suitable variant doesn't exist, run `/new-engine-command` first.

## Step 5 — Project state sync

Any persistent value the widget displays must be stored on `Project` (not just local widget state) so it survives save/load. Add the field to the appropriate struct in `src/project/`.

## Verification

- [ ] Widget renders without panic in `cargo run`
- [ ] Interacting with the widget does NOT spam the ring buffer (verify with a counter or xrun log)
- [ ] Value is preserved after `File → Save` and reopen
- [ ] No hardcoded colours — all from `theme::`
- [ ] `cargo check` clean with no new warnings
