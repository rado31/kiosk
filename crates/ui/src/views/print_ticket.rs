use egui::{
    Align2, Button, FontFamily, FontId, Frame, RichText, Sense, Shadow, Stroke, StrokeKind, Ui,
    vec2,
};

use crate::{
    components::keyboard,
    i18n::t,
    state::State,
    theme::{colors, corners},
};

const DISPLAY_WIDTH: f32 = 200.0;
const DISPLAY_HEIGHT: f32 = 60.0;
const DISPLAY_FONT_SIZE: f32 = 32.0;

pub fn show(state: &mut State, ui: &mut Ui) {
    let ctx = ui.ctx().clone();

    let card = Frame::new()
        .inner_margin(32)
        .corner_radius(corners::LARGE)
        .fill(colors::WHITE)
        .shadow(Shadow {
            offset: [0, 2],
            blur: 8,
            spread: 0,
            color: colors::SHADOW,
        });

    card.show(ui, |ui| {
        ui.set_width(ui.available_width());

        ui.vertical_centered(|ui| {
            let title = RichText::new(t(&state.lang, "enter_booking_number"))
                .size(24.0)
                .family(FontFamily::Name("bold".into()))
                .color(colors::BLACK);

            ui.label(title);
            ui.add_space(30.0);

            let (rect, response) =
                ui.allocate_exact_size(vec2(DISPLAY_WIDTH, DISPLAY_HEIGHT), Sense::CLICK);

            let border_color = if state.keyboard_visible {
                colors::PRIMARY
            } else {
                colors::BORDER
            };

            ui.painter().rect(
                rect,
                corners::MEDIUM,
                colors::INPUT_BG,
                Stroke::new(1.0, border_color),
                StrokeKind::Inside,
            );

            let (text, text_color) = if state.print_ticket.code.is_empty() {
                ("XXXXXX", colors::FG_PLACEHOLDER)
            } else {
                (state.print_ticket.code.as_str(), colors::FG)
            };

            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                text,
                FontId::new(DISPLAY_FONT_SIZE, FontFamily::Proportional),
                text_color,
            );

            if response.clicked() {
                state.keyboard_visible = true;
            }

            ui.add_space(30.0);

            let btn_label = RichText::new(t(&state.lang, "print"))
                .size(20.0)
                .family(FontFamily::Name("bold".into()))
                .color(colors::WHITE);

            let btn = Button::new(btn_label)
                .min_size(vec2(200.0, 60.0))
                .fill(colors::BTN_PRIMARY_BG)
                .corner_radius(12);

            if ui.add(btn).clicked() {
                log::debug!("{}", state.print_ticket.code);
            }
        });
    });

    keyboard::show(
        &mut state.keyboard_visible,
        &mut state.print_ticket.code,
        Some(6),
        &ctx,
    );
}
