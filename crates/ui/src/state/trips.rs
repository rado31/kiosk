use std::{sync::mpsc::Receiver, time::Instant};

use api::{stations::Station, trips::Trip};

#[derive(Default, PartialEq)]
pub enum TripKind {
    #[default]
    OneWay,
    Round,
}

pub struct State {
    pub kind: TripKind,
    pub source: Option<Station>,
    pub destination: Option<Station>,
    pub selected: bool,
    pub has_error: bool,
    searched_at: Option<Instant>,
    is_fetching: bool,
    data: Option<Vec<Trip>>,
    receiver: Option<Receiver<Option<Vec<Trip>>>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            kind: TripKind::OneWay,
            source: None,
            destination: None,
            selected: false,
            has_error: false,
            searched_at: None,
            is_fetching: false,
            data: None,
            receiver: None,
        }
    }
}

impl State {
    pub fn search_on_cooldown(&self) -> bool {
        self.searched_at
            .is_some_and(|t| t.elapsed().as_secs_f32() < 5.0)
    }

    pub fn mark_searched(&mut self) {
        self.searched_at = Some(Instant::now());
    }

    pub fn get_source(&self) -> Option<&Station> {
        self.source.as_ref()
    }

    pub fn get_destination(&self) -> Option<&Station> {
        self.destination.as_ref()
    }

    pub fn get_trips(&self) -> Option<&Vec<Trip>> {
        self.data.as_ref()
    }

    pub fn is_fetching(&self) -> bool {
        self.is_fetching
    }

    pub fn start_fetching(&mut self, receiver: Receiver<Option<Vec<Trip>>>) {
        self.is_fetching = true;
        self.has_error = false;
        self.data = None;
        self.receiver = Some(receiver);
    }

    pub fn take_receiver(&mut self) -> Option<Receiver<Option<Vec<Trip>>>> {
        self.receiver.take()
    }

    pub fn set_result(&mut self, data: Option<Vec<Trip>>) {
        self.has_error = data.is_none();
        self.data = data;
        self.is_fetching = false;
        self.receiver = None;
    }
}
