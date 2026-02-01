use egui::Ui;

use crate::app::{State, i18n::t};

pub struct PrintTicket<'a> {
    state: &'a mut State,
}

impl<'a> PrintTicket<'a> {
    pub fn new(state: &'a mut State) -> Self {
        Self { state }
    }

    pub fn show(&self, ui: &mut Ui) {
        ui.label(t(self.state.lang, "print_ticket_page"));

        ui.add_space(10.0);
    }
}
