use super::*;

pub fn show(state: &mut State, ui: &mut Ui) {
    ui.label(t(state.language, "refund_page"));

    ui.add_space(10.0);
}
