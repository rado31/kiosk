use std::sync::mpsc;

use super::{UpdateMessage, UpdateStatus, routes::Route};

mod lang;
mod pnrs;
mod trip;

pub use lang::Language;
pub use pnrs::PnrCounts;
pub use trip::TripType;

#[derive(Default)]
pub struct State {
    pub route: Route,
    pub lang: Language,
    pub update_status: UpdateStatus,
    pub update_receiver: Option<mpsc::Receiver<UpdateMessage>>,
    pub trip_type: TripType,
    pub pnr_counts: PnrCounts,
}

impl State {
    pub fn toggle_lang(&mut self) {
        self.lang = if self.lang == Language::Turkmen {
            Language::Russian
        } else {
            Language::Turkmen
        };
    }

    pub fn is_turkmen_lang(&self) -> bool {
        matches!(self.lang, Language::Turkmen)
    }

    pub fn set_one_way_trip(&mut self) {
        self.trip_type = TripType::OneWay;
    }

    pub fn set_round_trip(&mut self) {
        self.trip_type = TripType::RoundTrip;
    }

    pub fn is_one_way_trip(&self) -> bool {
        matches!(self.trip_type, TripType::OneWay)
    }
}
