use std::fmt::format;

use egui::{vec2, Align, Align2, Button, Color32, Image, Label, Layout, TextEdit, Ui};
use egui_flex::{item, Flex, FlexAlign, FlexAlignContent, FlexJustify};
use turingrs::turing_machine::TuringMachineExecutor;

use crate::TuringApp;

use super::button::{self, button, button_image, label, label_colored, text_edit_single};

// show the control part of the ui
pub fn ui(app: &mut TuringApp, ui: &mut Ui) {

    ui.columns_const(|[left, center, right]| {

        Flex::horizontal()
        .h_full()
        .align_items(FlexAlign::Center)
        .show(left, |flex| {

            let field = text_edit_single(flex.style_mut(),&mut app.input);
            let update = button(flex.style_mut(), "Update");
        
            flex.add(item().shrink(), field);

            if flex.add(item(), update).clicked() {
                app.update_input();
            }
        });

        Flex::horizontal()
        .h_full()
        .align_items(FlexAlign::Center)
        .justify(FlexJustify::Center)
        .show(center, |flex| {

            let play = button_image(flex.style_mut(),egui::include_image!("../../assets/play.png"));
            let pause = button_image(flex.style_mut(),egui::include_image!("../../assets/pause.png"));
            let reset = button_image(flex.style_mut(),egui::include_image!("../../assets/reset.png"));
            let next = button_image(flex.style_mut(),egui::include_image!("../../assets/next.png"));

            flex.add(item(),play);
            flex.add(item(),pause);
            if flex.add(item(),reset).clicked() {
                app.update_input();
            };
            if flex.add(item(),next).clicked() {
                app.next();
            };
        });

        Flex::horizontal()
        .h_full()
        .align_items(FlexAlign::Center)
        .justify(FlexJustify::SpaceAround)
        .show(right, |flex| {
            
            let steps = label(flex.style_mut(), &format!("Steps : {}", app.count));
            flex.add(item(), steps);

            if app.is_accepted.is_some() {
                let result = match app.is_accepted.unwrap() {
                    true => label_colored(flex.style_mut(), "Accepted", Color32::GREEN),
                    false => label_colored(flex.style_mut(), "Refused", Color32::RED),
                };
                flex.add(item(), result);
            }
        }); 
    });
}
