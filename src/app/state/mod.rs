use super::{services::updater::NewUpdate, views::View};

pub mod language;
mod modal;
mod pnrs;
mod stations;
mod trip;

#[derive(Default)]
pub struct State {
    pub lang: language::State,
    pub view: View,
    pub new_update: NewUpdate,
    pub pnr_counts: pnrs::State,
    pub modal: modal::State,
    pub stations: stations::State,
    pub trip: trip::State,
}

impl State {
    pub fn reset(&mut self) {
        self.pnr_counts = pnrs::State::default();
        self.stations = stations::State::default();
        self.trip = trip::State::default();
    }

    pub fn go_to(&mut self, view: View) {
        self.view = view;
    }
}
