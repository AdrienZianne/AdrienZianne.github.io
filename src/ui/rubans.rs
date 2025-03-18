use egui::{
    scroll_area::ScrollBarVisibility, Color32, CornerRadius, FontFamily, FontId, Frame, Label, Layout, Margin, Rect, RichText, ScrollArea, Sense, Stroke, Ui
};

use egui_infinite_scroll::{self, InfiniteScroll};
use unicode_segmentation::UnicodeSegmentation;
use crate::TuringApp;
use super::constant::Constant;

// show the rubans part of the gui
pub fn ui(app: &mut TuringApp, ui: &mut Ui) {

    let rubans_count = 2;
    
    let frame = Frame::new()
        .inner_margin(Margin::same(10))
        .outer_margin(Margin::same(0))
        .corner_radius(CornerRadius::same(5))
        .stroke(Stroke::new(1.0, Color32::WHITE))
        .show(ui, |ui: &mut Ui| {

            ui.vertical(|ui| {

                ui.spacing_mut().item_spacing = (5.0, 8.0).into();

                let width = ui.available_width();
                let left_width = width - (((width+5.0) / 35.0) + 2.0 + (1 - ((width+5.0) / 35.0).floor() as usize%2) as f32).floor() * 35.0 + 5.0;
                ScrollArea::horizontal()
                    .enable_scrolling(false)
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                    .horizontal_scroll_offset(-left_width/2.0)
                    .show(ui, |ui| {
                        for i in 0..rubans_count {
                            ruban(app, ui, i, width);
                        }
                    })
            });
        });
    
    ui.painter().rect_stroke(
        Rect::from_center_size(
            frame.response.rect.center(),
            (30.0, (rubans_count * 30 + (rubans_count - 1) * 5) as f32).into()
        ),
        CornerRadius::ZERO,
        Stroke::new(5.0, Constant::BORDER2),
        egui::StrokeKind::Outside
    );
}

// display a ruban
fn ruban(app: &mut TuringApp, ui: &mut Ui, index: usize, width: f32) {

    ui.horizontal(|ui| {

        let mut square_count = ((width+5.0) / 35.0) + 2.0;
        square_count += if square_count as usize%2==0 {1.0} else {0.0};
        let p = (square_count as i32/2) - app.turing.rubindex[0] as i32;
        let input = format!("ç{}$", &app.turing.input);


        for i in 0..square_count as i32 {
            if index == 0 && p <= i as i32 && i as i32 - p < input.graphemes(true).count() as i32 {
                draw_square(ui, input.chars().nth((i as i32-p) as usize).unwrap());
            } else {
                draw_square(ui, ' ');
            }
        }
    });
}

// display a square of a ruban
fn draw_square(ui: &mut Ui, t: char) {
    Frame::new().fill(Constant::FOREGROUND).show(ui, |ui| {
        let (rect, res) = ui.allocate_exact_size((30.0, 30.0).into(), Sense::empty());

        ui.put(
            rect,
            Label::new(
                RichText::new(t)
                    .font(FontId {
                        family: FontFamily::Name("Roboto".into()),
                        size: Constant::TEXT_SIZE,
                    })
                    .color(Color32::WHITE),
            ),
        );
    });
}