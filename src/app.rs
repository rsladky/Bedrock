use eframe::egui;
use crate::bridge::Bridge;
use crate::engine::AudioEngine;
use crate::project::project::Project;
use crate::ui::{UiState, ActiveView};

pub struct BedrockApp {
    pub project: Project,
    pub bridge: Bridge,
    pub ui_state: UiState,
    _engine: AudioEngine,
}

impl BedrockApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let project = Project::default();
        let (bridge, engine_handle) = Bridge::new();
        let engine = AudioEngine::new(engine_handle, &project);
        crate::ui::theme::apply_theme(&cc.egui_ctx);
        Self {
            project,
            bridge,
            ui_state: UiState::default(),
            _engine: engine,
        }
    }
}

impl eframe::App for BedrockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Drain audio events
        while let Ok(event) = self.bridge.event_rx.pop() {
            crate::bridge::events::handle_event(event, &mut self.ui_state, &mut self.project);
        }

        // Read snapshot
        self.ui_state.snapshot = self.bridge.read_snapshot();

        // Top toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            crate::ui::toolbar::show(ui, &mut self.bridge, &mut self.ui_state, &mut self.project);
        });

        // Bottom mixer
        egui::TopBottomPanel::bottom("mixer").min_height(180.0).show(ctx, |ui| {
            crate::ui::mixer::show(ui, &mut self.bridge, &mut self.ui_state, &mut self.project);
        });

        // Left channel rack
        egui::SidePanel::left("channel_rack").min_width(180.0).max_width(240.0).show(ctx, |ui| {
            crate::ui::channel_rack::show(ui, &mut self.bridge, &mut self.ui_state, &mut self.project);
        });

        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(self.ui_state.active_view == ActiveView::StepSequencer, "Step Seq").clicked() {
                    self.ui_state.active_view = ActiveView::StepSequencer;
                }
                if ui.selectable_label(self.ui_state.active_view == ActiveView::PianoRoll, "Piano Roll").clicked() {
                    self.ui_state.active_view = ActiveView::PianoRoll;
                }
                if ui.selectable_label(self.ui_state.active_view == ActiveView::Playlist, "Playlist").clicked() {
                    self.ui_state.active_view = ActiveView::Playlist;
                }
            });
            ui.separator();
            match self.ui_state.active_view {
                ActiveView::StepSequencer => crate::ui::step_sequencer::show(ui, &mut self.bridge, &mut self.ui_state, &mut self.project),
                ActiveView::PianoRoll => crate::ui::piano_roll::show(ui, &mut self.bridge, &mut self.ui_state, &mut self.project),
                ActiveView::Playlist => crate::ui::playlist::show(ui, &mut self.bridge, &mut self.ui_state, &mut self.project),
            }
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }
}
