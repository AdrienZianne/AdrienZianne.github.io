use egui::{
    vec2, Button, Color32, FontId, Image, ImageSource, Label, Margin, RichText, Stroke, Style, TextEdit, Ui
};

use super::constant::Constant;

// text button with styling
pub fn button<'a>(style: &mut Style, text: &str) -> Button<'a> {
    style.spacing.button_padding = vec2(10.0, 5.0);
    Button::new(
        RichText::new(text)
            .font(FontId {
                family: egui::FontFamily::Name("Roboto".into()),
                size: Constant::TEXT_SIZE,
            })
            .color(Color32::WHITE),
    )
    .stroke(Stroke::new(1.0, Color32::WHITE))
    .fill(Constant::BACKGROUND)
    .corner_radius(10.0)
}

// image button with styling
pub fn button_image<'a>(style: &mut Style, source: ImageSource<'a>) -> Button<'a> {
    style.spacing.button_padding = vec2(10.0, 5.0);
    Button::image(Image::new(source).fit_to_exact_size((24.0, 24.0).into()))
        .stroke(Stroke::new(1.0, Color32::WHITE))
        .fill(Constant::BACKGROUND)
        .corner_radius(10.0)
}

// label with styling
pub fn label(style: &mut Style, text: &str) -> Label {
    Label::new(
        RichText::new(text)
            .font(FontId {
                family: egui::FontFamily::Name("Roboto".into()),
                size: Constant::TEXT_SIZE,
            })
            .color(Color32::WHITE),
    )
}

// label with styling and custom colour
pub fn label_colored(style: &mut Style, text: &str, color: Color32) -> Label {
    Label::new(
        RichText::new(text)
            .font(FontId {
                family: egui::FontFamily::Name("Roboto".into()),
                size: Constant::TEXT_SIZE,
            })
            .color(color),
    )
}

pub fn text_edit_single<'a>(style: &mut Style, text: &'a mut String) -> TextEdit<'a> {
    style.visuals.clip_rect_margin = 1.0;
    TextEdit::singleline(text)
        .clip_text(true)
        .background_color(Constant::BACKGROUND2)
        .margin(Margin::symmetric(4, 5))
        .vertical_align(egui::Align::Center)
        .font(FontId {
            family: egui::FontFamily::Name("Roboto".into()),
            size: Constant::TEXT_SIZE,
        })
}
