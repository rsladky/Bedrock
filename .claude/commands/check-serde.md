Verify that `$ARGUMENTS` is correctly serializable and survives a serde round-trip.

## Step 1 — Find the type

Search for `struct $ARGUMENTS` or `enum $ARGUMENTS` in `src/`. Read the file containing it in full.

## Step 2 — Derive macro checklist

Every serializable type must have all three derives:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
```

Check:
- [ ] `Serialize` and `Deserialize` are derived (or manually implemented)
- [ ] `Default` is derived or implemented (needed for `#[serde(default)]` on fields)
- [ ] `use serde::{Serialize, Deserialize};` is in scope (or `serde::` is fully qualified)

## Step 3 — Reachability from `Project`

Trace the ownership chain from `Project` (in `src/project/project.rs`) down to `$ARGUMENTS`. If the type is not reachable, it won't be saved/loaded automatically.

Expected chains:
- `Project.channels: Vec<Channel>` → instrument config
- `Project.patterns: Vec<Pattern>` → step grid + piano roll
- `Project.arrangement: Arrangement` → clips
- `Project.mixer: Vec<MixerChannel>` → fader values

If `$ARGUMENTS` is not reachable, add it to the appropriate parent struct.

## Step 4 — Forbidden non-serializable types

Confirm none of these appear as fields (directly or transitively):

| Forbidden | Why | Fix |
|---|---|---|
| `Arc<T>` | Not serializable | Store the inner `T`; reconstruct `Arc` on load |
| `Mutex<T>` / `RwLock<T>` | Not serializable | Same as above |
| `egui::*` types | UI-only, no serde | Keep UI state separate from project state |
| `Box<dyn Trait>` | Requires custom impl | Use an enum instead |
| Raw pointers | Meaningless after reload | Never store pointers in project data |

## Step 5 — New-field backward compatibility

Any field added to an existing serialized struct **must** carry `#[serde(default)]` so old save files without that field still load:

```rust
#[serde(default)]
pub new_field: f32,
```

## Step 6 — Add a round-trip test

Add this test to `src/project/project.rs` (or the file containing `$ARGUMENTS`):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_$ARGUMENTS_serde_round_trip() {
        let original = $ARGUMENTS::default(); // or construct a representative value
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: $ARGUMENTS = serde_json::from_str(&json).expect("deserialize");
        // Add field-by-field assertions here, e.g.:
        // assert_eq!(original.some_field, restored.some_field);
        let _ = restored; // suppress unused warning if no assertions added yet
    }
}
```

Run with `cargo test` and confirm it passes.

## Verification checklist

- [ ] All three derives present
- [ ] No forbidden non-serializable types in the field tree
- [ ] New fields have `#[serde(default)]`
- [ ] Type is reachable from `Project`
- [ ] Round-trip test passes (`cargo test`)
- [ ] `cargo check` clean
