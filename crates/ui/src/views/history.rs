use egui::{Frame, Shadow, Ui};

use crate::{
    state::State,
    theme::{colors, corners},
};

pub fn show(_state: &mut State, ui: &mut Ui) {
    Frame::new()
        .inner_margin(10)
        .corner_radius(corners::LARGE)
        .fill(colors::WHITE)
        .shadow(Shadow {
            offset: [0, 2],
            blur: 8,
            spread: 0,
            color: colors::SHADOW,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| ui.label("history page"));
        });
}
