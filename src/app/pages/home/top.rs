use super::*;
use crate::debug;

pub const LEFT_WIDTH: f32 = 300.0;
pub const RIGHT_WIDTH: f32 = 100.0;
const HEIGHT: f32 = 40.0;

impl<'a> Home<'a> {
    pub fn show_top_left(&mut self, ui: &mut Ui) {
        Frame::new()
            .inner_margin(5)
            .corner_radius(8)
            .stroke(Stroke::new(1.0, colors::BORDER))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (rect_1, response_1) =
                        ui.allocate_exact_size(vec2(LEFT_WIDTH / 2.0, HEIGHT), Sense::CLICK);

                    let (rect_2, response_2) =
                        ui.allocate_exact_size(vec2(LEFT_WIDTH / 2.0, HEIGHT), Sense::CLICK);

                    if response_1.clicked() {
                        self.state.set_one_way();
                    }

                    if response_2.clicked() {
                        self.state.set_round_trip();
                    }

                    let curr_rect = if self.state.is_one_way() {
                        rect_1
                    } else {
                        rect_2
                    };

                    let current_x = ui.ctx().animate_value_with_time(
                        ui.id().with("tab_indicator"),
                        curr_rect.min.x,
                        0.2,
                    );

                    let animated_rect = Rect::from_min_size(
                        pos2(current_x, curr_rect.min.y),
                        vec2(curr_rect.width(), curr_rect.height()),
                    );

                    ui.painter().rect_filled(
                        animated_rect,
                        corners::MEDIUM,
                        colors::BTN_PRIMARY_BG,
                    );

                    let (text_color_1, text_color_2) = if self.state.is_one_way() {
                        (colors::WHITE, colors::BLACK)
                    } else {
                        (colors::BLACK, colors::WHITE)
                    };

                    // 2. Draw text labels on TOP
                    ui.painter().text(
                        rect_1.center(),
                        Align2::CENTER_CENTER,
                        t(self.state.language, "one_way"),
                        FontId::proportional(14.0),
                        text_color_1,
                    );

                    ui.painter().text(
                        rect_2.center(),
                        Align2::CENTER_CENTER,
                        t(self.state.language, "round_trip"),
                        FontId::proportional(14.0),
                        text_color_2,
                    );
                });
            });
    }

    pub fn show_top_right(&mut self, ui: &mut Ui) {
        let (rect, response) = ui.allocate_exact_size(vec2(RIGHT_WIDTH, HEIGHT), Sense::CLICK);

        ui.painter().rect(
            rect,
            corners::MEDIUM,
            colors::BTN_PRIMARY_TEXT,
            Stroke::new(1.0, colors::BORDER),
            StrokeKind::Outside,
        );

        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            format!("{}  1", t(self.state.language, "pnr")),
            FontId::proportional(14.0),
            colors::BLACK,
        );

        let modal_id = ui.id().with("pnr_modal_state");
        let mut modal_is_open = ui.data(|d| d.get_temp::<bool>(modal_id).unwrap_or(false));

        if response.clicked() {
            ui.data_mut(|d| d.insert_temp(modal_id, true));
        }

        if modal_is_open {
            let close = Modal::new("pnr_modal").show(self.ctx, |ui| {});

            if close {
                ui.data_mut(|d| d.insert_temp(modal_id, false));
            };
        }
    }
}
