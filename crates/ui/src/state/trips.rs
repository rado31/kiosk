use std::{sync::mpsc::Receiver, time::Instant};

use api::{stations::Station, trips::Trip};

type TripResult = (Option<Vec<Trip>>, Option<Vec<Trip>>);

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
    pub inbound_has_error: bool,
    searched_at: Option<Instant>,
    is_fetching: bool,
    outbound_data: Option<Vec<Trip>>,
    inbound_data: Option<Vec<Trip>>,
    receiver: Option<Receiver<TripResult>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            kind: TripKind::OneWay,
            source: None,
            destination: None,
            selected: false,
            has_error: false,
            inbound_has_error: false,
            searched_at: None,
            is_fetching: false,
            outbound_data: None,
            inbound_data: None,
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

    pub fn get_outbound(&self) -> Option<&Vec<Trip>> {
        self.outbound_data.as_ref()
    }

    pub fn get_inbound(&self) -> Option<&Vec<Trip>> {
        self.inbound_data.as_ref()
    }

    pub fn is_fetching(&self) -> bool {
        self.is_fetching
    }

    pub fn start_fetching(&mut self, receiver: Receiver<TripResult>) {
        self.is_fetching = true;
        self.has_error = false;
        self.inbound_has_error = false;
        self.outbound_data = None;
        self.inbound_data = None;
        self.receiver = Some(receiver);
    }

    pub fn take_receiver(&mut self) -> Option<Receiver<TripResult>> {
        self.receiver.take()
    }

    pub fn set_result(&mut self, outbound: Option<Vec<Trip>>, inbound: Option<Vec<Trip>>) {
        self.has_error = outbound.is_none();
        self.inbound_has_error = inbound.is_none() && self.kind == TripKind::Round;
        self.outbound_data = outbound;
        self.inbound_data = inbound;
        self.is_fetching = false;
        self.receiver = None;
    }
}
