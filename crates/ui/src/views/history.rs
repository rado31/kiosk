use egui::{Frame, Stroke, Ui};

use crate::{state::State, theme::colors};

pub fn show(_state: &mut State, ui: &mut Ui) {
    Frame::new()
        .inner_margin(10)
        .corner_radius(12)
        .fill(colors::WHITE)
        .stroke(Stroke::new(1.0, colors::BORDER))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| ui.label("history page"));
        });
}
