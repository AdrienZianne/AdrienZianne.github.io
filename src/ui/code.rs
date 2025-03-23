
use super::{button::button, constant::Constant};
use crate::TuringApp;
use egui::{
    style::Selection, vec2, CentralPanel, Color32, CornerRadius, Frame, Label, Layout, Margin, RichText, ScrollArea, Stroke, TextEdit, Ui, Visuals
};
use egui_flex::{item, Flex, FlexDirection};
use turingrs::parser::parse_turing_machine;

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

            Flex::new()
            .direction(FlexDirection::Horizontal)
            .show(ui, |flex| {
                let compile = button(flex.style_mut(), "Compile");
                let load_file = button(flex.style_mut(), "Load file");
                if flex.add(item(), compile).clicked() {
                    app.compile();
                }
                if flex.add(item(), load_file).clicked() {

                }
            });

            ScrollArea::vertical().show(ui, |ui| {
                let mut lines_number: String = String::from("");
                
                // println!("{} {}", ui.available_width(), ui.available_width() / Constant::TEXT_SIZE);

                for (i, s) in app.code.lines().enumerate() {
                    lines_number += &((i+1).to_string() + &String::from("\n".repeat(1)))
                }

                Frame::new().fill(Constant::FOREGROUND).show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        let numbers = Label::new(
                            RichText::new(lines_number)
                                .color(Color32::WHITE)
                                .line_height(Some(Constant::TEXT_SIZE + 3.0))
                                .font(Constant::get_small_font()),
                        )
                        .extend();

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
                                    top: 5,
                                    left: 3,
                                    right: 4,
                                    ..Default::default()
                                })
                                .show(ui, |ui| {
                                    ui.allocate_ui_with_layout(vec2(20.0, ui.available_height()), Layout::top_down(egui::Align::Max), |ui| {
                                        ui.add(numbers);
                                        ui.allocate_space(vec2(20.0, ui.available_height()))
                                    });
                                })
                        });

                        ui.ctx().set_visuals(Visuals {
                            selection: Selection {
                                stroke: Stroke::new(2.0, Color32::RED),
                                ..Default::default()
                            },
                            ..Default::default()
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
