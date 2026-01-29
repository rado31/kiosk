#![allow(dead_code)]

#[derive(PartialEq)]
enum TripKind {
    OneWay,
    Round,
}

pub struct Trip {
    kind: TripKind,
    source: u16,
    destination: u16,
}

impl Default for Trip {
    fn default() -> Self {
        Self {
            kind: TripKind::OneWay,
            source: 17,
            destination: 0,
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

    pub fn get_source(&self) -> u16 {
        self.source
    }

    pub fn get_destination(&self) -> u16 {
        self.destination
    }

    pub fn set_source(&mut self, source: u16) {
        self.source = source;
    }

    pub fn set_destination(&mut self, destination: u16) {
        self.destination = destination;
    }
}
