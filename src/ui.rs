use constant::Constant;
use egui::{CentralPanel, Frame, Id, Margin, TopBottomPanel};

use crate::TuringApp;
mod rubans;
mod control;
mod graph;
mod code;
mod button;
pub mod turing;
pub mod constant;

pub fn show(app: &mut TuringApp, ctx: &egui::Context) {

    CentralPanel::default()
    .frame(Frame {
        outer_margin: Margin::same(0),
        inner_margin: Margin::same(10),
        fill: Constant::BACKGROUND,
        ..Default::default()
    })
    .show(ctx, |ui| {

        // Rubans and control
        TopBottomPanel::top(Id::new("Rubans"))
        .frame(Frame {
            outer_margin: Margin::same(0),
            inner_margin: Margin::same(0),
            ..Default::default()
        })
        .show_inside(ui, |ui| {
            ui.style_mut().spacing.item_spacing = (10.0,10.0).into();
            rubans::ui(app, ui);
            control::ui(app, ui);
        });

        // Code and/or Graph
        CentralPanel::default()
        .frame(Frame {
            outer_margin: Margin {
                top: 10,
                ..Default::default()
            },
            inner_margin: Margin::same(0),
            ..Default::default()
        })
        .show_inside(ui, |ui| {
            graph::ui(app, ui);
            code::ui(app, ui);
        });
    });
}