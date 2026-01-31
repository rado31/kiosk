use super::{State, components, pages};

#[derive(Default, Copy, Clone, PartialEq)]
pub enum View {
    #[default]
    Home,
    PrintTicket,
    Refund,
    Seats,
    History,
}

pub fn view(state: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) {
    if state.view != View::Seats {
        components::Menu::new(state).show(ui);
    }

    match state.view {
        View::Home => pages::Home::new(state, ctx).show(ui),
        View::PrintTicket => pages::print_ticket::show(state, ui),
        View::Refund => pages::refund::show(state, ui),
        View::Seats => pages::seats::show(state, ui),
        View::History => pages::History::new(state).show(ui),
    }
}
