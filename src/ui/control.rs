use egui::{vec2, Align, Align2, Button, Color32, Image, Label, Layout, TextEdit, Ui};
use egui_flex::{item, Flex, FlexAlign, FlexAlignContent, FlexJustify};

use crate::TuringApp;

use super::button::{self, button, button_image, label, label_colored, text_edit_single};

// show the control part of the ui
pub fn ui(app: &mut TuringApp, ui: &mut Ui) {

    ui.columns_const(|[left, center, right]| {

        Flex::horizontal()
        .h_full()
        .align_items(FlexAlign::Center)
        .show(left, |flex| {
            let mut text = String::new();

            let field = text_edit_single(flex.style_mut(), &mut app.old_turing.input);
            let update = button(flex.style_mut(), "Update");
        
            flex.add(item().shrink(), field);
            flex.add(item(), update);
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
                app.old_turing.rubindex[0] = 0;
            };
            if flex.add(item(),next).clicked() {
                app.old_turing.rubindex[0] += 1;
            };
        });

        Flex::horizontal()
        .h_full()
        .align_items(FlexAlign::Center)
        .justify(FlexJustify::SpaceAround)
        .show(right, |flex| {
            
            let steps = label(flex.style_mut(), "Steps : 0");
            flex.add(item(), steps);

            if true {
                let result = label_colored(flex.style_mut(), "Accepted", Color32::GREEN);
                flex.add(item(), result);
            }
        }); 
    });
}
