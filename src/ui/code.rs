use std::default;

use super::constant::Constant;
use crate::TuringApp;
use egui::{
    CentralPanel, Color32, CornerRadius, Frame, Id, Label, Margin, RichText, ScrollArea, SidePanel,
    Stroke, TextEdit, Ui, frame, vec2,
};

// show the code part of the gui
pub fn ui(app: &mut TuringApp, ui: &mut Ui) {
    let remainder = ui.available_width() - 35.0;

    CentralPanel::default()
        .frame(Frame {
            stroke: Stroke::new(1.0, Constant::BORDER),
            outer_margin: Margin::symmetric(2, 0),
            inner_margin: Margin::same(10),
            corner_radius: CornerRadius::same(5),
            ..Default::default()
        })
        .show_inside(ui, |ui| {
            ui.set_width(remainder);

            ScrollArea::vertical().show(ui, |ui| {
                let mut lines_number: String = String::from("");
                for i in 1..app.code.lines().count() + 1 {
                    lines_number += &(i.to_string() + &String::from('\n'))
                }

                Frame::new().fill(Constant::FOREGROUND).show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        let numbers = Label::new(
                            RichText::new(lines_number)
                                .color(Color32::WHITE)
                                .font(Constant::get_code_font()),
                        )
                        .halign(egui::Align::Max);

                        Frame::new().
                        fill(Color32::WHITE)
                        .inner_margin(Margin {
                            right: 1,
                            ..Default::default()
                        })
                        .show(ui, |ui| {
                            Frame::new()
                                .fill(Constant::FOREGROUND)
                                .inner_margin(Margin {
                                    top: 2,
                                    left: 3,
                                    right: 4,
                                    ..Default::default()
                                })
                                .show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        ui.add(numbers);
                                        ui.allocate_space(vec2(20.0, ui.available_height()))
                                    });
                                })
                        });

                        let code_edit = TextEdit::multiline(&mut app.code)
                            .background_color(Color32::TRANSPARENT)
                            .code_editor()
                            .text_color(Color32::WHITE)
                            .font(Constant::get_code_font());

                        ui.add_sized(ui.available_size() - (0.0, 0.0).into(), code_edit);
                    });
                });
            });
        });
}
