use egui::Context;

use crate::{
    components::calendar::{Calendar, CalendarKind},
    state::{State, modal::Modal},
};

pub fn show(state: &mut State, ctx: &Context) {
    if state.modal == Modal::OneWayCalendar {
        let mut cal = Calendar::new(
            "one_way_cal",
            &mut state.calendar,
            &state.lang,
            ctx,
            CalendarKind::OneWay,
        );

        if cal.show() {
            state.modal = Modal::Closed;
        }
    }

    if state.modal == Modal::RoundTripCalendar {
        let mut cal = Calendar::new(
            "round_trip_cal",
            &mut state.calendar,
            &state.lang,
            ctx,
            CalendarKind::RoundTrip,
        );

        if cal.show() {
            state.modal = Modal::Closed;
        }
    }
}
