use super::*;

pub fn show(state: &mut State, ui: &mut Ui) {
    ui.label(t(state.lang, "print_ticket_page"));

    ui.add_space(10.0);
}
