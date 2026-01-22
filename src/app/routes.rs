use super::{State, components, pages};

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
        Route::Home => pages::home::show(state, ui),
        Route::PrintTicket => pages::print_ticket::show(state, ui),
        Route::Refund => pages::refund::show(state, ui),
        Route::Seats => pages::seats::show(state, ui),
    }
}
