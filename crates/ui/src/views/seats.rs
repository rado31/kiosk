use egui::Ui;

use crate::{i18n::t, state::State};

pub fn show(state: &State, ui: &mut Ui) {
    ui.label(t(&state.lang, "seats_page"));
    ui.add_space(10.0);
}
