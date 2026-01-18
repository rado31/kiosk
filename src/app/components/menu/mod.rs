mod button;

use egui::{
    Color32, CornerRadius, Image, ImageSource, RichText, Sense, Stroke, StrokeKind, Ui, UiBuilder,
    include_image, vec2,
};

use crate::{
    app::{Language, State, constants, i18n::t, routes::Route},
    utils,
};
use button::Button;

pub fn show(state: &mut State, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new(t(state.language, "terminal_title"))
                .size(32.0)
                .strong()
                .color(constants::BLACK),
        )
    });

    let buttons = [
        Button::new(
            include_image!("../../../assets/ticket-check.svg"),
            Route::Home,
            constants::PRIMARY,
        ),
        Button::new(
            include_image!("../../../assets/printer-check.svg"),
            Route::PrintTicket,
            constants::BTN_GREEN,
        ),
        Button::new(
            include_image!("../../../assets/ticket-x.svg"),
            Route::Refund,
            constants::BTN_RED,
        ),
    ];

    ui.add_space(30.0);

    ui.horizontal(|ui| {
        ui.style_mut().spacing.item_spacing.x = 10.0;

        for button in buttons.iter() {
            button.show(ui, state);
        }
    });
}
