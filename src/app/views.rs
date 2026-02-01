use super::{
    State, components,
    pages::{History, Home, PrintTicket, Refund, Seats},
};

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
        View::Home => Home::new(state, ctx).show(ui),
        View::PrintTicket => PrintTicket::new(state).show(ui),
        View::Refund => Refund::new(state).show(ui),
        View::Seats => Seats::new(state).show(ui),
        View::History => History::new(state).show(ui),
    }
}
