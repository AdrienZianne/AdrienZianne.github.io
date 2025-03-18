use super::constant::Constant;
use crate::TuringApp;
use egui::{CornerRadius, Frame, Id, Margin, SidePanel, Stroke, TextEdit, Ui};

// show the code part of the gui
pub fn ui(app: &mut TuringApp, ui: &mut Ui) {
    let remainder = ui.available_width() - 35.0;

    SidePanel::right(Id::new("Code"))
        .frame(Frame {
            stroke: Stroke::new(1.0, Constant::BORDER),
            outer_margin: Margin::symmetric(2, 0),
            inner_margin: Margin::same(10),
            corner_radius: CornerRadius::same(5),
            ..Default::default()
        })
        .resizable(false)
        .min_width(100.0)
        .max_width(ui.available_width())
        .show_inside(ui, |ui| {
            ui.set_width(remainder);

            ui.add_sized(ui.available_size() - (0.0,0.0).into(),TextEdit::multiline(&mut app.turing.code));
        }
    );
}
