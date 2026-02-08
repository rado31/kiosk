use egui::{Align, Context, Frame, Layout, Stroke, Ui};

use crate::{state::State, theme::colors};

mod calendar;
mod panel;
mod stations;

pub fn show(state: &mut State, ctx: &Context, ui: &mut Ui) {
    let frame = Frame::new()
        .inner_margin(10)
        .corner_radius(12)
        .fill(colors::WHITE)
        .stroke(Stroke::new(1.0, colors::BORDER));

    frame.show(ui, |ui| {
        ui.set_width(ui.available_width());

        ui.columns_const(|[col1, _col2, col3]| {
            col1.horizontal(|ui| panel::top_left(state, ui));
            col3.with_layout(Layout::right_to_left(Align::Min), |ui| {
                panel::top_right(state, ctx, ui);
            });
        });

        ui.add_space(20.0);

        panel::bottom(state, ui);
        stations::show(state, ctx);
        calendar::show(state, ctx);
    });
}
