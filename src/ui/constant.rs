use egui::{Color32, FontId};

// Constant used in the app
#[non_exhaustive]
pub struct Constant;

impl Constant {
    pub const BACKGROUND: Color32 = Color32::from_rgb(62, 62, 62);
    pub const BACKGROUND2: Color32 = Color32::from_rgb(41, 41, 41);
    pub const BORDER: Color32 = Color32::WHITE;
    pub const BORDER2: Color32 = Color32::from_gray(175);
    pub const FOREGROUND: Color32 = Color32::from_rgb(109, 109, 109);
    pub const ARROW: Color32 = Color32::WHITE;
    pub const SELECTED: Color32 = Color32::RED;
    pub const TEXT_SIZE: f32 = 16.0;
    pub const SMALL_TEXT_SIZE: f32 = 12.0;
    pub const CREP: f32 = 10000.0;
    pub const CSPRING: f32 = 100.0;
    pub const L: f32 = 200.0;
    pub const MAX_FORCE: f32 = 100000.0;
    pub fn get_code_font() -> FontId {
        FontId {
            family: egui::FontFamily::Name("Roboto".into()),
            size: Constant::TEXT_SIZE,
        }
    }
    pub fn get_small_font() -> FontId {
        FontId {
            family: egui::FontFamily::Name("Roboto".into()),
            size: Constant::SMALL_TEXT_SIZE,
        }
    }
}
