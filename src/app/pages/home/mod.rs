use super::*;

mod top;

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

                let space = ui.available_width() - top::LEFT_WIDTH - top::RIGHT_WIDTH;

                ui.horizontal(|ui| {
                    self.show_top_left(ui);
                    ui.add_space(space);
                    self.show_top_right(ui);
                })
            });
    }
}
