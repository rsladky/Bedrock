Add a new `EngineCommand` variant named `$ARGUMENTS` following the 3-file workflow below. Read each file before editing it.

## Step 1 — Declare the variant (`src/bridge/commands.rs`)

Read `src/bridge/commands.rs`, then add the new variant to the `EngineCommand` enum. Include any payload fields needed. Example:

```rust
SetVolume { channel: usize, value: f32 },
```

## Step 2 — Handle in the audio thread (`src/engine/audio_thread.rs`)

Read `src/engine/audio_thread.rs`, then add a match arm inside `AudioState::process_command()`.

**Hot-path safety rules (mandatory):**
- No heap allocation (`Vec::new`, `Box::new`, `String::new`, `.clone()` of heap types, etc.)
- No mutex locks (`Mutex::lock`, `parking_lot::Mutex::lock`)
- No I/O (`println!`, `eprintln!`, file ops, network)
- No blocking calls (`sleep`, channel `recv`)
- Prefer simple field assignments or arithmetic only
- If you need to store data, use pre-allocated fields on `AudioState`

## Step 3 — Send from the GUI (`src/app.rs` or a `src/ui/` panel)

Read the relevant UI file, then call:

```rust
let _ = self.bridge.send(EngineCommand::$ARGUMENTS { /* fields */ });
```

Gate the send with `.changed()` or `.clicked()` — never send every frame unconditionally.
Also update the corresponding field on `self.project` **before** sending, so the project file stays in sync.

## Verification checklist

- [ ] `match cmd` in `process_command()` is still exhaustive (no missing arms, no `_ => {}` hiding bugs)
- [ ] No `panic!`, `unwrap()`, or `expect()` added inside `process_command()` or any function it calls
- [ ] If the command carries a `Box<T>`, dereference it (`*val`) before use — don't store the Box
- [ ] `cargo check` passes with no new warnings
- [ ] Run `cargo run` and exercise the new command path manually
