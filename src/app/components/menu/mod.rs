use super::*;

mod button;

use button::Button;

pub fn show(state: &mut State, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new(t(state.lang, "terminal_title"))
                .size(32.0)
                .color(colors::BLACK),
        )
    });

    let buttons = [
        Button::new(
            include_image!("../../../assets/ticket-check.svg"),
            Route::Home,
            colors::PRIMARY,
        ),
        Button::new(
            include_image!("../../../assets/printer-check.svg"),
            Route::PrintTicket,
            colors::BTN_GREEN,
        ),
        Button::new(
            include_image!("../../../assets/ticket-x.svg"),
            Route::Refund,
            colors::BTN_RED,
        ),
        Button::new(
            include_image!("../../../assets/ticket-check.svg"),
            Route::History,
            colors::SECONDARY,
        ),
    ];

    ui.add_space(30.0);

    ui.horizontal(|ui| {
        ui.style_mut().spacing.item_spacing.x = 10.0;

        for button in buttons.iter() {
            button.show(ui, state);
        }
    });

    ui.add_space(20.0);
}
