use crate::ui::UiState;
use crate::project::project::Project;

#[derive(Debug, Clone)]
pub enum EngineEvent {
    BeatTick { beat: u32 },
    Xrun,
    StepAdvanced { step: usize },
}

pub fn handle_event(event: EngineEvent, ui_state: &mut UiState, _project: &mut Project) {
    match event {
        EngineEvent::BeatTick { beat: _ } => {}
        EngineEvent::Xrun => {
            log::warn!("Audio xrun detected");
        }
        EngineEvent::StepAdvanced { step } => {
            ui_state.snapshot.active_step = step;
        }
    }
}
