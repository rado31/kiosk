#![allow(dead_code)]

use crate::app::services::api::stations::types::Station;

#[derive(PartialEq)]
enum TripKind {
    OneWay,
    Round,
}

pub struct Trip {
    kind: TripKind,
    source: Option<Station>,
    destination: Option<Station>,
}

impl Default for Trip {
    fn default() -> Self {
        Self {
            kind: TripKind::OneWay,
            source: None,
            destination: None,
        }
    }
}

impl Trip {
    pub fn set_one_way(&mut self) {
        self.kind = TripKind::OneWay;
    }

    pub fn set_round(&mut self) {
        self.kind = TripKind::Round;
    }

    pub fn is_one_way(&self) -> bool {
        matches!(self.kind, TripKind::OneWay)
    }

    pub fn set_source(&mut self, station: Station) {
        self.source = Some(station);
    }

    pub fn set_destination(&mut self, station: Station) {
        self.destination = Some(station);
    }

    pub fn get_source(&self) -> Station {
        self.source
            .clone()
            .unwrap_or(Station::new("Nirden".to_owned(), "Откуда".to_owned()))
    }
}
