use std::sync::mpsc;

use super::{UpdateMessage, UpdateStatus, routes::Route};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Language {
    #[default]
    Turkmen,
    Russian,
}

#[derive(Default, PartialEq)]
pub enum TripType {
    #[default]
    OneWay,
    RoundTrip,
}

#[derive(Default)]
pub struct State {
    pub route: Route,
    pub language: Language,
    pub update_status: UpdateStatus,
    pub update_receiver: Option<mpsc::Receiver<UpdateMessage>>,
    pub trip_type: TripType,
}

impl State {
    pub fn toggle_language(&mut self) {
        self.language = if self.language == Language::Turkmen {
            Language::Russian
        } else {
            Language::Turkmen
        };
    }

    pub fn is_turkmen(&self) -> bool {
        matches!(self.language, Language::Turkmen)
    }

    pub fn set_one_way(&mut self) {
        self.trip_type = TripType::OneWay;
    }

    pub fn set_round_trip(&mut self) {
        self.trip_type = TripType::RoundTrip;
    }

    pub fn is_one_way(&self) -> bool {
        matches!(self.trip_type, TripType::OneWay)
    }
}
