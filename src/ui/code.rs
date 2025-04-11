use std::{ffi::OsStr, fs, path::Path};

use super::{button::button, constant::Constant};
use crate::TuringApp;
use egui::{
    CentralPanel, Color32, CornerRadius, Frame, Label, Layout, Margin, RichText, ScrollArea,
    Stroke, TextEdit, Ui, Visuals,
    style::Selection, Response,
    text::{Fonts, LayoutJob},
    vec2,
};
use poll_promise::Promise;
use rfd::{AsyncFileDialog, FileHandle};
use egui_flex::{Flex, FlexDirection, item};
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

            ui.scope(|ui| {

                ui.ctx().set_visuals(Visuals {
                    ..Default::default()
                });

                Flex::new()
                    .direction(FlexDirection::Horizontal)
                    .show(ui, |flex| {
                        let compile = button(flex.style_mut(), "Compile");
                        let load_file_button = button(flex.style_mut(), "Load file");
                        if flex.add(item(), compile).clicked() {
                            app.compile();
                        }

                        
                        let res = flex.add(item(), load_file_button);
                        load_file(app, res);
                    });
            });

            ScrollArea::vertical().show(ui, |ui| {
                Frame::new().fill(Constant::FOREGROUND).show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        Frame::new()
                            .fill(Color32::WHITE)
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
                                        let mut lines_number: String = String::from("");

                                        let number_width = ui.fonts(|f| {
                                            let x = f.layout_job(LayoutJob::simple_singleline(
                                                app.code.lines().count().to_string(),
                                                Constant::get_code_font(),
                                                Constant::FOREGROUND,
                                            ));
                                            x.rect.width()
                                        });

                                        for (i, s) in app.code.lines().enumerate() {
                                            let row_per_line = ui.fonts(|f| {
                                                let x = f.layout_job(LayoutJob::simple(
                                                    s.to_string(),
                                                    Constant::get_code_font(),
                                                    Constant::FOREGROUND,
                                                    ui.available_width() - number_width - 16.0,
                                                ));
                                                x.rows.iter().count()
                                            });
                                            lines_number += &((i + 1).to_string()
                                                + &String::from("\n".repeat(row_per_line)))
                                        }

                                        let numbers = Label::new(
                                            RichText::new(lines_number)
                                                .color(Color32::WHITE)
                                                .line_height(Some(Constant::TEXT_SIZE + 3.0))
                                                .font(Constant::get_small_font()),
                                        )
                                        .extend();

                                        ui.allocate_ui_with_layout(
                                            vec2(number_width, ui.available_height()),
                                            Layout::top_down(egui::Align::Max),
                                            |ui| {
                                                ui.add(numbers);
                                                ui.allocate_space(vec2(
                                                    number_width,
                                                    ui.available_height(),
                                                ))
                                            },
                                        );
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

#[cfg(not(target_arch = "wasm32"))]
fn load_file(app: &mut TuringApp, res: Response) {

    use rfd::FileDialog;

    
    if res.clicked() {
        // if let Some(path) = FileDialog::new().add_filter("tm", &["tm"]).pick_file() {
        //     app.code = fs::read_to_string(path).expect("cannot read file");
        // }

        app.promise = Some(Promise::spawn_thread("load_file",    || FileDialog::new().add_filter("ext", &["tm"]).pick_file()));
    }

    if let Some(promise) = &app.promise {
        
        if let Some(path) = (promise).ready() {
            app.code = fs::read_to_string(path.as_ref().unwrap()).expect("cannot read file");
        } else {

        }
    }

}

#[cfg(target_arch = "wasm32")]
fn load_file(app: &mut TuringApp, res: Response) {
    use std::path::PathBuf;


    if res.clicked() {

        app.promise_wasm = Some(Promise::spawn_local(async move { AsyncFileDialog::new().add_filter("ext", &["tm"]).pick_file().await}));
    }

    if let Some(promise) = &app.promise {
        
        if let Some(path) = (promise).ready() {
            app.code = fs::read_to_string(path.as_ref().unwrap()).expect("cannot read file");
        } else {

        }
    }

}