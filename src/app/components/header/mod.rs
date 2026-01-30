use super::*;

mod left;
mod mid;
mod right;

pub struct Header<'a> {
    state: &'a mut State,
    ctx: &'a Context,
}

impl<'a> Header<'a> {
    pub fn new(state: &'a mut State, ctx: &'a Context) -> Self {
        Self { state, ctx }
    }

    pub fn show(&mut self) {
        let frame = Frame::new()
            .fill(colors::WHITE)
            .stroke(Stroke::new(1.0, colors::BORDER))
            .inner_margin(Margin::symmetric(30, 10));

        let top_panel = TopBottomPanel::top("top_panel")
            .frame(frame)
            .show_separator_line(false);

        top_panel.show(self.ctx, |ui| {
            ui.columns_const(|[col1, col2, col3]| {
                col1.horizontal_centered(|ui| self.render_left(ui));
                col2.centered_and_justified(|ui| self.render_mid(ui));
                col3.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    self.render_right(ui);
                });
            });
        });
    }
}
