use std::sync::mpsc::{Receiver, TryRecvError};

use api::trips::{Seat, TrainWagon};

#[derive(Clone)]
pub struct SeatSelection {
    pub wagon_idx: usize,
    pub seat_id: u32,
    pub seat_label: String,
}

#[derive(Default, PartialEq)]
pub enum SeatsLeg {
    #[default]
    Outbound,
    Inbound,
}

#[derive(Default, PartialEq)]
pub enum SlideDir {
    #[default]
    None,
    Prev,
    Next,
}

impl SlideDir {
    pub fn as_f32(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Prev => -1.0,
            Self::Next => 1.0,
        }
    }
}

#[derive(Default)]
pub struct State {
    outbound_is_fetching: bool,
    pub outbound_has_error: bool,
    outbound_wagons: Option<Vec<TrainWagon>>,
    outbound_receiver: Option<Receiver<Option<Vec<TrainWagon>>>>,
    pub outbound_selected: Vec<Option<SeatSelection>>,

    // Inbound pre-fetched data (parallel fetch while user selects outbound seats)
    inbound_is_fetching: bool,
    pub inbound_has_error: bool,
    inbound_wagons: Option<Vec<TrainWagon>>,
    inbound_receiver: Option<Receiver<Option<Vec<TrainWagon>>>>,
    pub inbound_selected: Vec<Option<SeatSelection>>,

    pub current_wagon: usize,
    pub slide_dir: SlideDir,
    pub leg: SeatsLeg,

    // Cached layout for the current wagon; rebuilt when needs_organize is set.
    pub organized_seats: Vec<Vec<Vec<Seat>>>,
    pub needs_organize: bool,
}

impl State {
    pub fn init(&mut self, passenger_count: usize) {
        *self = Self {
            inbound_selected: vec![None; passenger_count],
            ..Default::default()
        };
    }

    /// Polls the outbound receiver. Returns `true` if a repaint is needed.
    pub fn poll_outbound(&mut self) -> bool {
        let Some(rx) = self.outbound_receiver.take() else {
            return false;
        };

        match rx.try_recv() {
            Ok(result) => {
                self.set_result(result);
                true
            }
            Err(TryRecvError::Empty) => {
                self.start_fetching(rx);
                true
            }
            Err(TryRecvError::Disconnected) => {
                self.set_result(None);
                false
            }
        }
    }

    /// Polls the inbound receiver. Returns `true` if a repaint is needed.
    pub fn poll_inbound(&mut self) -> bool {
        let Some(rx) = self.inbound_receiver.take() else {
            return false;
        };

        match rx.try_recv() {
            Ok(result) => {
                self.set_inbound_result(result);
                true
            }
            Err(TryRecvError::Empty) => {
                self.start_fetching_inbound(rx);
                true
            }
            Err(TryRecvError::Disconnected) => {
                self.set_inbound_result(None);
                false
            }
        }
    }

    pub fn is_fetching(&self) -> bool {
        self.outbound_is_fetching
    }

    pub fn get_wagons(&self) -> Option<&Vec<TrainWagon>> {
        self.outbound_wagons.as_ref()
    }

    pub fn current_wagon(&self) -> Option<&TrainWagon> {
        self.outbound_wagons.as_ref()?.get(self.current_wagon)
    }

    pub fn wagon_count(&self) -> usize {
        self.outbound_wagons.as_ref().map_or(0, |w| w.len())
    }

    pub fn start_fetching(&mut self, rx: Receiver<Option<Vec<TrainWagon>>>) {
        self.outbound_is_fetching = true;
        self.outbound_has_error = false;
        self.outbound_wagons = None;
        self.outbound_receiver = Some(rx);
    }

    pub fn set_result(&mut self, result: Option<Vec<TrainWagon>>) {
        self.outbound_has_error = result.is_none();
        self.outbound_wagons = result;
        self.outbound_is_fetching = false;
        self.needs_organize = true;
    }

    pub fn start_fetching_inbound(&mut self, rx: Receiver<Option<Vec<TrainWagon>>>) {
        self.inbound_is_fetching = true;
        self.inbound_has_error = false;
        self.inbound_wagons = None;
        self.inbound_receiver = Some(rx);
    }

    pub fn set_inbound_result(&mut self, result: Option<Vec<TrainWagon>>) {
        self.inbound_has_error = result.is_none();
        self.inbound_wagons = result;
        self.inbound_is_fetching = false;
    }

    /// Switch the active leg to inbound: saves outbound selections and loads inbound wagons.
    pub fn switch_to_inbound(&mut self) {
        self.outbound_selected = std::mem::take(&mut self.inbound_selected);
        self.outbound_wagons = self.inbound_wagons.take();
        self.outbound_has_error = self.inbound_has_error;
        self.outbound_is_fetching = self.inbound_is_fetching;
        self.inbound_has_error = false;
        self.inbound_is_fetching = false;
        self.inbound_selected = vec![None; self.outbound_selected.len()];
        self.current_wagon = 0;
        self.slide_dir = SlideDir::None;
        self.leg = SeatsLeg::Inbound;
        self.needs_organize = true;
    }

    pub fn prev_wagon(&mut self) {
        if self.current_wagon > 0 {
            self.current_wagon -= 1;
            self.slide_dir = SlideDir::Prev;
            self.needs_organize = true;
        }
    }

    pub fn next_wagon(&mut self) {
        let max = self.wagon_count().saturating_sub(1);

        if self.current_wagon < max {
            self.current_wagon += 1;
            self.slide_dir = SlideDir::Next;
            self.needs_organize = true;
        }
    }

    pub fn assign_seat(&mut self, wagon_idx: usize, seat_id: u32, seat_label: String) {
        if let Some(slot) = self.inbound_selected.iter_mut().find(|s| s.is_none()) {
            *slot = Some(SeatSelection {
                wagon_idx,
                seat_id,
                seat_label,
            });
        }
    }

    pub fn unassign_seat(&mut self, wagon_idx: usize, seat_id: u32) {
        for slot in self.inbound_selected.iter_mut() {
            if slot
                .as_ref()
                .is_some_and(|s| s.wagon_idx == wagon_idx && s.seat_id == seat_id)
            {
                *slot = None;
                break;
            }
        }
    }

    pub fn is_seat_selected(&self, wagon_idx: usize, seat_id: u32) -> bool {
        self.inbound_selected.iter().any(|s| {
            s.as_ref()
                .is_some_and(|s| s.wagon_idx == wagon_idx && s.seat_id == seat_id)
        })
    }

    pub fn all_assigned(&self) -> bool {
        !self.inbound_selected.is_empty() && self.inbound_selected.iter().all(|s| s.is_some())
    }
}
