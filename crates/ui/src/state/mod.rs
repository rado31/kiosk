use crate::{i18n::Language, views::View};

pub mod calendar;
pub mod modal;
pub mod passengers;
pub mod stations;
pub mod trips;
pub mod update;

#[derive(Default)]
pub struct State {
    view: View,
    pub lang: Language,
    pub update: update::State,
    pub modal: modal::Modal,
    pub passengers: passengers::State,
    pub calendar: calendar::State,
    pub stations: stations::State,
    pub trips: trips::State,
}

impl State {
    pub fn reset(&mut self) {
        self.passengers = passengers::State::default();
        self.calendar = calendar::State::default();
        self.stations = stations::State::default();
        self.trips = trips::State::default();
    }

    pub fn current_view(&self) -> View {
        self.view
    }

    pub fn go_to(&mut self, view: View) {
        self.view = view;
    }
}
