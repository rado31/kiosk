use super::{services::updater::NewUpdate, views::View};

pub mod calendar;
pub mod language;
mod modal;
mod pnrs;
mod stations;
mod trip;

#[derive(Default)]
pub struct State {
    view: View,
    pub lang: language::State,
    pub new_update: NewUpdate,
    pub pnr_counts: pnrs::State,
    pub modal: modal::State,
    pub stations: stations::State,
    pub trip: trip::State,
    pub calendar: calendar::State,
}

impl State {
    pub fn reset(&mut self) {
        self.pnr_counts = pnrs::State::default();
        self.stations = stations::State::default();
        self.trip = trip::State::default();
        self.calendar = calendar::State::default();
    }

    pub fn current_view(&self) -> View {
        self.view
    }

    pub fn go_to(&mut self, view: View) {
        self.view = view;
    }
}
