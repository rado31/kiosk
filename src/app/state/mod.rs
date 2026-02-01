use super::{services::updater::NewUpdate, views::View};

mod lang;
mod modal;
mod pnrs;
mod stations;
mod trip;

pub use lang::Language;
use modal::Modal;
use pnrs::PnrCounts;
use trip::Trip;

#[derive(Default)]
pub struct State {
    pub lang: Language,
    pub view: View,
    pub new_update: NewUpdate,
    pub trip: Trip,
    pub pnr_counts: PnrCounts,
    pub modal: Modal,
    pub stations: stations::State,
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
