use egui::{RichText, Ui, include_image};

use crate::app::{State, constants::colors, i18n::t, views::View};

mod button;

use button::Button;

pub struct Menu<'a> {
    state: &'a mut State,
}

impl<'a> Menu<'a> {
    pub fn new(state: &'a mut State) -> Self {
        Self { state }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            let title = RichText::new(t(self.state.lang.get(), "terminal_title"))
                .size(32.0)
                .color(colors::BLACK);

            ui.label(title)
        });

        let buttons = [
            Button::new(
                include_image!("../../../assets/ticket-check.svg"),
                View::Home,
                colors::PRIMARY,
            ),
            Button::new(
                include_image!("../../../assets/printer-check.svg"),
                View::PrintTicket,
                colors::BTN_GREEN,
            ),
            Button::new(
                include_image!("../../../assets/ticket-x.svg"),
                View::Refund,
                colors::BTN_RED,
            ),
            Button::new(
                include_image!("../../../assets/ticket-check.svg"),
                View::History,
                colors::SECONDARY,
            ),
        ];

        ui.add_space(30.0);

        ui.horizontal(|ui| {
            ui.style_mut().spacing.item_spacing.x = 10.0;

            for button in buttons.iter() {
                button.render(ui, self.state);
            }
        });

        ui.add_space(20.0);
    }
}
