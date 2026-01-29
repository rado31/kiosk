use std::sync::mpsc;

use super::{UpdateMessage, UpdateStatus, routes::Route};

mod lang;
mod modal;
mod pnrs;
mod trip;

pub use lang::Language;
use modal::Modal;
pub use pnrs::PnrCounts;
use trip::Trip;

#[derive(Default)]
pub struct State {
    pub route: Route,
    pub lang: Language,
    pub update_status: UpdateStatus,
    pub update_receiver: Option<mpsc::Receiver<UpdateMessage>>,
    pub trip: Trip,
    pub pnr_counts: PnrCounts,
    pub modal: Modal,
}

impl State {
    // Language
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

    // Modals
    pub fn close_modal(&mut self) {
        self.modal = Modal::Closed;
    }

    pub fn open_pnr_counts_modal(&mut self) {
        self.modal = Modal::PnrCounts;
    }

    pub fn open_stations_modal(&mut self) {
        self.modal = Modal::Stations;
    }

    pub fn is_pnr_counts_modal_opened(&self) -> bool {
        matches!(self.modal, Modal::PnrCounts)
    }

    pub fn is_stations_modal_opened(&self) -> bool {
        matches!(self.modal, Modal::Stations)
    }
}
