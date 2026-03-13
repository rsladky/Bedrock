use eframe::egui::{self, Color32, Visuals};

pub const BG_DARK: Color32 = Color32::from_rgb(0x1e, 0x1e, 0x1e);
pub const BG_MID: Color32 = Color32::from_rgb(0x2a, 0x2a, 0x2a);
pub const BG_LIGHT: Color32 = Color32::from_rgb(0x3a, 0x3a, 0x3a);
pub const FL_ORANGE: Color32 = Color32::from_rgb(0xff, 0x78, 0x00);
pub const FL_ORANGE_DIM: Color32 = Color32::from_rgb(0x99, 0x48, 0x00);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0xe0, 0xe0, 0xe0);
pub const TEXT_DIM: Color32 = Color32::from_rgb(0x80, 0x80, 0x80);
pub const GREEN: Color32 = Color32::from_rgb(0x00, 0xcc, 0x44);
pub const YELLOW: Color32 = Color32::from_rgb(0xff, 0xcc, 0x00);
pub const RED: Color32 = Color32::from_rgb(0xff, 0x22, 0x22);

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = dark_visuals();
    ctx.set_style(style);
}

fn dark_visuals() -> Visuals {
    let mut v = Visuals::dark();
    v.panel_fill = BG_DARK;
    v.window_fill = BG_DARK;
    v.faint_bg_color = BG_MID;
    v.extreme_bg_color = Color32::from_rgb(0x14, 0x14, 0x14);
    v.override_text_color = Some(TEXT_PRIMARY);
    v.selection.bg_fill = FL_ORANGE;
    v.widgets.noninteractive.bg_fill = BG_MID;
    v.widgets.inactive.bg_fill = BG_LIGHT;
    v.widgets.hovered.bg_fill = Color32::from_rgb(0x4a, 0x4a, 0x4a);
    v.widgets.active.bg_fill = FL_ORANGE;
    v
}
