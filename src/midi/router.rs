use crate::bridge::commands::EngineCommand;

pub fn midi_to_command(msg: &[u8], channel_id: usize) -> Option<EngineCommand> {
    if msg.len() < 3 {
        return None;
    }
    let status = msg[0] & 0xF0;
    match status {
        0x90 if msg[2] > 0 => Some(EngineCommand::NoteOn {
            channel_id,
            pitch: msg[1],
            velocity: msg[2],
        }),
        0x80 | 0x90 => Some(EngineCommand::NoteOff {
            channel_id,
            pitch: msg[1],
        }),
        _ => None,
    }
}
