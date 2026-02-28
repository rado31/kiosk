use std::sync::mpsc::Receiver;

use api::trips::TrainWagon;

pub struct SeatSelection {
    pub wagon_idx: usize,
    pub seat_id: u32,
    pub seat_label: String,
}

#[derive(Default)]
pub struct State {
    is_fetching: bool,
    pub has_error: bool,
    wagons: Option<Vec<TrainWagon>>,
    receiver: Option<Receiver<Option<Vec<TrainWagon>>>>,
    pub current_wagon: usize,
    pub selected: Vec<Option<SeatSelection>>,
    /// Set to ±1.0 when the wagon changes; consumed by the grid renderer.
    pub slide_dir: f32,
}

impl State {
    pub fn init(&mut self, passenger_count: usize) {
        self.selected = (0..passenger_count).map(|_| None).collect();
        self.current_wagon = 0;
        self.slide_dir = 0.0;
        self.has_error = false;
        self.wagons = None;
        self.is_fetching = false;
        self.receiver = None;
    }

    pub fn is_fetching(&self) -> bool {
        self.is_fetching
    }

    pub fn get_wagons(&self) -> Option<&Vec<TrainWagon>> {
        self.wagons.as_ref()
    }

    pub fn current_wagon(&self) -> Option<&TrainWagon> {
        self.wagons.as_ref()?.get(self.current_wagon)
    }

    pub fn wagon_count(&self) -> usize {
        self.wagons.as_ref().map_or(0, |w| w.len())
    }

    pub fn start_fetching(&mut self, rx: Receiver<Option<Vec<TrainWagon>>>) {
        self.is_fetching = true;
        self.has_error = false;
        self.wagons = None;
        self.receiver = Some(rx);
    }

    pub fn take_receiver(&mut self) -> Option<Receiver<Option<Vec<TrainWagon>>>> {
        self.receiver.take()
    }

    pub fn set_result(&mut self, result: Option<Vec<TrainWagon>>) {
        self.has_error = result.is_none();
        self.wagons = result;
        self.is_fetching = false;
    }

    pub fn prev_wagon(&mut self) {
        if self.current_wagon > 0 {
            self.current_wagon -= 1;
            self.slide_dir = -1.0;
        }
    }

    pub fn next_wagon(&mut self) {
        let max = self.wagon_count().saturating_sub(1);

        if self.current_wagon < max {
            self.current_wagon += 1;
            self.slide_dir = 1.0;
        }
    }

    pub fn assign_seat(&mut self, wagon_idx: usize, seat_id: u32, seat_label: String) {
        if let Some(slot) = self.selected.iter_mut().find(|s| s.is_none()) {
            *slot = Some(SeatSelection {
                wagon_idx,
                seat_id,
                seat_label,
            });
        }
    }

    pub fn unassign_seat(&mut self, wagon_idx: usize, seat_id: u32) {
        for slot in self.selected.iter_mut() {
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
        self.selected.iter().any(|s| {
            s.as_ref()
                .is_some_and(|s| s.wagon_idx == wagon_idx && s.seat_id == seat_id)
        })
    }

    pub fn all_assigned(&self) -> bool {
        !self.selected.is_empty() && self.selected.iter().all(|s| s.is_some())
    }
}
