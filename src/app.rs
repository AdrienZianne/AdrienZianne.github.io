use std::collections::BTreeMap;

use egui::{FontData, FontDefinitions, FontFamily, Rect, Visuals};
use egui_extras::install_image_loaders;
use::turingrs::turing_machine::TuringMachine;

use crate::{turing::Turing, ui::{self, constant::Constant}};

pub struct TuringApp {
    pub old_turing: Turing,
    pub turing: TuringMachine,
    pub code: String,
    pub graph_rect: Rect,
    pub is_stable: bool,
}

impl Default for TuringApp {
    fn default() -> Self {
        Self {
            old_turing: Turing::default(),
            turing: TuringMachine::new(1),
            graph_rect: Rect::ZERO,
            is_stable: true,
            code: String::from("")
        }
    }
}

impl TuringApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        cc.egui_ctx.set_visuals(Visuals {
            window_fill: Constant::BACKGROUND,
            panel_fill: Constant::BACKGROUND,
            extreme_bg_color: Constant::BACKGROUND2,
            ..Default::default()
        });
        
        // cc.egui_ctx.set_debug_on_hover(true);

        load_font(cc);

        Default::default()
    }
}

impl eframe::App for TuringApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        install_image_loaders(ctx);

        ui::show(self, ctx);
    }
}

fn load_font(cc: &eframe::CreationContext<'_>) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "Roboto".into(),
        FontData::from_static(include_bytes!("../assets/fonts/Roboto.ttf")).into(),
    );

    let mut newfam = BTreeMap::new();
    newfam.insert(FontFamily::Name("Roboto".into()), vec!["Roboto".to_owned()]);
    fonts.families.append(&mut newfam);

    cc.egui_ctx.set_fonts(fonts);
}
