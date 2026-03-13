use eframe::egui;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Bedrock")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Bedrock",
        native_options,
        Box::new(|cc| Ok(Box::new(crate::app::BedrockApp::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!("eframe error: {e}"))
}

mod app;
mod engine;
mod project;
mod bridge;
mod ui;
mod midi;
