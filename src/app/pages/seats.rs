use egui::{Context, Ui};

use crate::app::{State, i18n::t};

pub fn show(state: &mut State, ui: &mut Ui) {
    ui.label(t(state.lang, "seats_page"));
}
