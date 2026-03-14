pub mod commands;
pub mod events;
pub mod shared_state;

use commands::EngineCommand;
use events::EngineEvent;
use parking_lot::Mutex;
use rtrb::{Consumer, Producer, RingBuffer};
use shared_state::EngineSnapshot;
use std::sync::Arc;

pub struct Bridge {
    pub cmd_tx: Producer<EngineCommand>,
    pub event_rx: Consumer<EngineEvent>,
    pub snapshot: Arc<Mutex<EngineSnapshot>>,
}

impl Bridge {
    pub fn new() -> (Self, EngineHandle) {
        let (cmd_tx, cmd_rx) = RingBuffer::new(256);
        let (event_tx, event_rx) = RingBuffer::new(256);
        let snapshot = Arc::new(Mutex::new(EngineSnapshot::default()));
        (
            Bridge {
                cmd_tx,
                event_rx,
                snapshot: Arc::clone(&snapshot),
            },
            EngineHandle {
                cmd_rx,
                event_tx,
                snapshot,
            },
        )
    }
    pub fn send(&mut self, cmd: EngineCommand) {
        let _ = self.cmd_tx.push(cmd);
    }
    pub fn read_snapshot(&self) -> EngineSnapshot {
        self.snapshot.lock().clone()
    }
}

pub struct EngineHandle {
    pub cmd_rx: Consumer<EngineCommand>,
    pub event_tx: Producer<EngineEvent>,
    pub snapshot: Arc<Mutex<EngineSnapshot>>,
}

impl EngineHandle {
    pub fn write_snapshot(&self, snap: EngineSnapshot) {
        *self.snapshot.lock() = snap;
    }
}
