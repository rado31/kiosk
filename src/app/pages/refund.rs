use super::*;

pub fn show(state: &mut State, ui: &mut Ui) {
    ui.label(t(state.lang, "refund_page"));

    ui.add_space(10.0);
}
