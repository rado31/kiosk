use egui::{Align2, Area, Context, Frame, Id, Order, Sense, Stroke, Ui, Vec2};

use crate::app::constants::{colors, corners};

pub struct Modal<'a> {
    id: &'a str,
    width: f32,
    is_closable: bool,
}

impl<'a> Modal<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            width: 400.0,
            is_closable: true,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn closable(mut self, is_closable: bool) -> Self {
        self.is_closable = is_closable;
        self
    }

    /// Shows the modal. Returns `true` if the modal was closed by clicking outside.
    /// When modal is open, all interaction with background widgets is blocked.
    pub fn open<F: FnOnce(&mut Ui)>(self, ctx: &Context, content: F) -> bool {
        let screen_rect = ctx.input(|i| i.viewport_rect());
        let mut closed = false;

        // Fullscreen overlay Area - blocks background and detects outside clicks
        Area::new(Id::new(format!("{}_overlay", self.id)))
            .order(Order::Foreground)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                ui.expand_to_include_rect(screen_rect);
                ui.painter_at(screen_rect)
                    .rect_filled(screen_rect, corners::ZERO, colors::OVERLAY);

                let response = ui.allocate_response(screen_rect.size(), Sense::CLICK);

                if self.is_closable && response.clicked() {
                    closed = true;
                }
            });

        // Modal window on top (Tooltip order is higher than Foreground)
        Area::new(Id::new(self.id))
            .order(Order::Tooltip)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                Frame::new()
                    .fill(colors::WHITE)
                    .corner_radius(corners::LARGE)
                    .stroke(Stroke::new(1.0, colors::BORDER))
                    .inner_margin(25.0)
                    .show(ui, |ui| {
                        ui.set_width(self.width);
                        content(ui);
                    });
            });

        closed
    }
}
