use egui::Ui;

use crate::app::{State, i18n::t};

pub struct Refund<'a> {
    state: &'a mut State,
}

impl<'a> Refund<'a> {
    pub fn new(state: &'a mut State) -> Self {
        Self { state }
    }

    pub fn show(&self, ui: &mut Ui) {
        ui.label(t(self.state.lang.get(), "refund_page"));

        ui.add_space(10.0);
    }
}
