mod button;

use egui::{
    Color32, CornerRadius, Image, ImageSource, RichText, Sense, Ui, UiBuilder, include_image, vec2,
};

use crate::{
    app::{State, routes::Route},
    utils,
};
use button::Button;

pub fn show(state: &mut State, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new("Terminal to buy railway tickets")
                .size(32.0)
                .strong()
                .color(Color32::BLACK),
        )
    });

    let buttons = [
        Button::new(
            include_image!("../../../assets/ticket-check.svg"),
            Route::Home,
            Color32::BLUE,
        ),
        Button::new(
            include_image!("../../../assets/printer-check.svg"),
            Route::PrintTicket,
            Color32::from_rgb(52, 199, 89),
        ),
        Button::new(
            include_image!("../../../assets/ticket-x.svg"),
            Route::Refund,
            Color32::from_rgb(255, 59, 48),
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
