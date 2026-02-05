use egui::Ui;

use crate::app::components::base::Modal;

use super::Home;

impl<'a> Home<'a> {
    pub fn show_calendar(&mut self) {
        let calendar = self.state.modal.is_one_way_trip_calendar()
            || self.state.modal.is_round_trip_calendar();

        if calendar {
            let should_close = Modal::new("calendar").width(800.0).open(self.ctx, |ui| {
                ui.label("calendar");
            });

            if should_close {
                self.state.modal.close();
            };
        }
    }
}
