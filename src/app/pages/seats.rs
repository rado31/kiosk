use super::*;

pub fn show(state: &mut State, ui: &mut Ui) {
    ui.label(t(state.language, "seats_page"));
}
