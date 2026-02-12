use egui::{Align, Context, Frame, Layout, Shadow, Ui};

use crate::{
    state::State,
    theme::{colors, corners},
};

mod calendar;
mod panel;
mod stations;

pub fn show(state: &mut State, ctx: &Context, ui: &mut Ui) {
    let frame = Frame::new()
        .inner_margin(10)
        .corner_radius(corners::LARGE)
        .fill(colors::WHITE)
        .shadow(Shadow {
            offset: [0, 2],
            blur: 8,
            spread: 0,
            color: colors::SHADOW,
        });

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
