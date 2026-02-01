#![allow(dead_code)]

use egui::{Frame, Stroke, Ui};

use crate::app::{State, constants::colors};

pub struct History<'a> {
    state: &'a mut State,
}

impl<'a> History<'a> {
    pub fn new(state: &'a mut State) -> Self {
        Self { state }
    }

    pub fn show(&mut self, ui: &mut Ui) {
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
}
