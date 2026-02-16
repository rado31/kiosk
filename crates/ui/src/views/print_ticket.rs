use egui::{Align, Frame, Shadow, Ui, vec2};

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
            Input::new(&mut state.print_ticket.code)
                .hint(t(&state.lang, "enter_ticket_code"))
                .bold()
                .font_size(32.0)
                .char_limit(6)
                .horizontal_align(Align::Center)
                .desired_size(vec2(200.0, 60.0))
                .show(ui);
        });
    });
}
