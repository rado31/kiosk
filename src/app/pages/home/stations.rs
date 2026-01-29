use super::*;

impl<'a> Home<'a> {
    pub fn show_stations(&mut self, ui: &mut Ui) {
        if self.state.is_stations_modal_opened() {
            let should_close = Modal::new("pnr_counts_modal").show(self.ctx, |ui| {
                ui.label("Stations modal");
            });

            if should_close {
                self.state.close_modal();
            };
        }
    }
}
