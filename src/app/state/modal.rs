use super::*;

#[derive(Default, PartialEq)]
pub enum Modal {
    #[default]
    Closed,
    PnrCounts,
    Stations,
}
