use egui_toast::Toasts;

use crate::{components::toast, i18n::Language, views::View};

pub mod calendar;
pub mod modal;
pub mod passengers;
pub mod stations;
pub mod trips;
pub mod print_ticket;
pub mod update;

pub struct State {
    view: View,
    pub lang: Language,
    pub update: update::State,
    pub modal: modal::Modal,
    pub passengers: passengers::State,
    pub calendar: calendar::State,
    pub toasts: Toasts,
    pub stations: stations::State,
    pub print_ticket: print_ticket::State,
    pub trips: trips::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            view: View::default(),
            lang: Language::default(),
            update: update::State::default(),
            modal: modal::Modal::default(),
            passengers: passengers::State::default(),
            calendar: calendar::State::default(),
            toasts: toast::create(),
            stations: stations::State::default(),
            print_ticket: print_ticket::State::default(),
            trips: trips::State::default(),
        }
    }
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
