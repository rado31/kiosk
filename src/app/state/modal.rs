#[derive(Default, PartialEq)]
pub enum Kind {
    #[default]
    Closed,
    PnrCounts,
    Source,
    Destination,
}

#[derive(Default)]
pub struct State {
    modal: Kind,
}

impl State {
    pub fn close(&mut self) {
        self.modal = Kind::Closed;
    }

    pub fn pnr_counts(&mut self) {
        self.modal = Kind::PnrCounts;
    }

    pub fn source(&mut self) {
        self.modal = Kind::Source;
    }

    pub fn destination(&mut self) {
        self.modal = Kind::Destination;
    }

    pub fn is_pnr_counts(&self) -> bool {
        matches!(self.modal, Kind::PnrCounts)
    }

    pub fn is_source(&self) -> bool {
        matches!(self.modal, Kind::Source)
    }

    pub fn is_destination(&self) -> bool {
        matches!(self.modal, Kind::Destination)
    }
}
