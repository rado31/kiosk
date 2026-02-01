use egui::{Align2, Area, Context, Frame, Id, Order, Stroke, Vec2};

use crate::app::constants::{colors, corners};

pub fn show(ctx: &Context) {
    Area::new(Id::new("toast"))
        .order(Order::Tooltip)
        .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 50.0))
        .show(ctx, |ui| {
            Frame::NONE
                .fill(colors::BTN_RED)
                .corner_radius(corners::MEDIUM)
                .stroke(Stroke::new(1.0, colors::BORDER))
                .inner_margin(25.0)
                .show(ui, |ui| {
                    ui.label("some text");
                });
        });
}
