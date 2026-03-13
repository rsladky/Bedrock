pub mod router;

use crate::bridge::Bridge;

pub struct MidiManager {
    // midir connection placeholder
    _conn: Option<Box<dyn std::any::Any + Send>>,
}

impl MidiManager {
    pub fn new(_bridge: &mut Bridge) -> Self {
        // Stub — midir setup would go here
        Self { _conn: None }
    }
}
