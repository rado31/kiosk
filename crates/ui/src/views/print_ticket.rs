use egui::{Align, Button, FontFamily, Frame, RichText, Shadow, Ui, vec2};

use crate::{
    components::input::Input,
    i18n::t,
    state::State,
    theme::{colors, corners},
};

pub fn show(state: &mut State, ui: &mut Ui) {
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

            Input::new(&mut state.print_ticket.code)
                .hint("XXXXXX")
                .font_size(32.0)
                .char_limit(6)
                .horizontal_align(Align::Center)
                .desired_size(vec2(200.0, 60.0))
                .show(ui);

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
                log::debug!("print button clicked");
            }
        });
    });
}
