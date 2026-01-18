use crate::app::{State, components, views};

#[derive(Default, Copy, Clone, PartialEq)]
pub enum Route {
    #[default]
    Home,
    PrintTicket,
    Refund,
    Seats,
}

pub fn router(state: &mut State, ui: &mut egui::Ui) {
    if state.current_route != Route::Seats {
        components::menu::show(state, ui);
    }

    match state.current_route {
        Route::Home => views::home::show(state, ui),
        Route::PrintTicket => views::print_ticket::show(state, ui),
        Route::Refund => views::refund::show(state, ui),
        Route::Seats => views::seats::show(state, ui),
    }
}
