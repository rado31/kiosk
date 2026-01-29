use super::*;

mod panel;
mod stations;

pub struct Home<'a> {
    state: &'a mut State,
    ctx: &'a Context,
}

impl<'a> Home<'a> {
    pub fn new(state: &'a mut State, ctx: &'a Context) -> Self {
        Self { state, ctx }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        Frame::new()
            .inner_margin(10)
            .corner_radius(12)
            .fill(colors::WHITE)
            .stroke(Stroke::new(1.0, colors::BORDER))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                ui.columns_const(|[col1, _col2, col3]| {
                    col1.horizontal(|ui| self.show_panel_top_left(ui));
                    col3.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        self.show_panel_top_right(ui);
                    });
                });

                ui.add_space(20.0);

                self.show_panel_bottom(ui);
                self.show_stations(ui);
            });
    }
}
