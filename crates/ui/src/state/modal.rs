#[derive(Default, PartialEq)]
pub enum Modal {
    #[default]
    Closed,
    PnrCounts,
    Source,
    Destination,
    OneWayCalendar,
    RoundTripCalendar,
}
