use crate::app::components::calendar::{Calendar, CalendarKind};

use super::Home;

impl<'a> Home<'a> {
    pub fn show_calendar(&mut self) {
        if self.state.modal.is_one_way_trip_calendar() {
            let mut cal = Calendar::new(
                "one_way_cal",
                &mut self.state.calendar,
                self.state.lang.get(),
                self.ctx,
                CalendarKind::OneWay,
            );

            if cal.show() {
                self.state.modal.close();
            }
        }

        if self.state.modal.is_round_trip_calendar() {
            let mut cal = Calendar::new(
                "round_trip_cal",
                &mut self.state.calendar,
                self.state.lang.get(),
                self.ctx,
                CalendarKind::RoundTrip,
            );

            if cal.show() {
                self.state.modal.close();
            }
        }
    }
}
