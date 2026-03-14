Audit `fill_buffer()` and its full call graph in `src/engine/audio_thread.rs` for real-time safety violations.

## Scope

Read and analyse every function reachable from `fill_buffer()`:

1. `fill_buffer()` itself
2. `AudioState::process_command()` — command drain loop
3. `sequencer.tick()` (in `src/engine/sequencer.rs`)
4. `voices.note_on()` / `voices.note_off()` (in `src/engine/synth.rs`)
5. `voices.render()` (in `src/engine/synth.rs`)
6. Any `write_snapshot()` or snapshot update block

## Forbidden patterns (flag as VIOLATION)

| Pattern | Why forbidden |
|---|---|
| `Vec::new()`, `push()`, `String::new()`, `.to_owned()`, `.clone()` on heap types | Allocates — can trigger the global allocator and block |
| `Mutex::lock()`, `RwLock::write()`, `parking_lot::Mutex::lock()` | Can block the audio thread → xrun |
| `println!`, `eprintln!`, `log::*`, file I/O, network | Syscalls with unpredictable latency |
| `thread::sleep`, `channel::recv` (blocking), `std::sync::mpsc::recv` | Blocks |
| `Box::new(...)` at runtime | Heap allocation |
| `unwrap()` / `expect()` on `Result` or `Option` that can realistically fail | Silent panic → process kill mid-audio |

## Accepted trade-off (do NOT flag)

- The `snapshot.lock()` write gated to every 512 frames is a known, documented trade-off. Note it in the report but do not mark it as a violation.

## Report format

Produce a structured report with exactly these four sections:

```
### VIOLATIONS
<list each finding: file, function, line, pattern, suggested fix>

### ACCEPTED TRADE-OFFS
<list known trade-offs with their justification>

### CLEAN
<list functions confirmed free of forbidden patterns>

### SUMMARY
<one-paragraph verdict: is the hot path safe to ship?>
```

If there are no violations, say so explicitly in VIOLATIONS.
