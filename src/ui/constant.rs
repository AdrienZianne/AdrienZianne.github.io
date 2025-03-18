use egui::Color32;


// Constant used in the app
#[non_exhaustive]
pub struct Constant;

impl Constant {
    pub const BACKGROUND: Color32 = Color32::from_rgb(62,62,62);
    pub const BACKGROUND2: Color32 = Color32::from_rgb(41,41,41);
    pub const BORDER: Color32 = Color32::WHITE;
    pub const BORDER2: Color32 = Color32::from_gray(175);
    pub const FOREGROUND: Color32 = Color32::from_rgb(109,109,109);
    pub const ARROW: Color32 = Color32::WHITE;
    pub const TEXT_SIZE: f32 = 16.0;
    pub const CREP: f32 = 1000.0;
    pub const CSPRING: f32 = 100.0;
    pub const L: f32 = 150.0;
}