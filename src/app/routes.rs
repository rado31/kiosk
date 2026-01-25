use super::{State, components, pages};

#[derive(Default, Copy, Clone, PartialEq)]
pub enum Route {
    #[default]
    Home,
    PrintTicket,
    Refund,
    Seats,
}

pub fn router(state: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) {
    if state.route != Route::Seats {
        components::menu::show(state, ui);
    }

    match state.route {
        Route::Home => pages::Home::new(state, ctx).show(ui),
        Route::PrintTicket => pages::print_ticket::show(state, ui),
        Route::Refund => pages::refund::show(state, ui),
        Route::Seats => pages::seats::show(state, ui),
    }
}
