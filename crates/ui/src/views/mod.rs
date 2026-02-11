use crate::{components, state::State};

mod history;
mod home;
mod print_ticket;
mod refund;
mod seats;
mod trips;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum View {
    #[default]
    Home,
    Trips,
    PrintTicket,
    Refund,
    Seats,
    History,
}

pub fn view(state: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) {
    if state.current_view() != View::Seats && state.current_view() != View::Trips {
        components::menu::show(state, ui);
    }

    match state.current_view() {
        View::Home => home::show(state, ctx, ui),
        View::Trips => trips::show(state, ctx, ui),
        View::PrintTicket => print_ticket::show(state, ui),
        View::Refund => refund::show(state, ui),
        View::Seats => seats::show(state, ui),
        View::History => history::show(state, ui),
    }
}
