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
        let btn_title = RichText::new(format!("{}  1", t(self.state.language, "pnr")))
            .size(14.0)
            .color(colors::BLACK);

        let btn = Button::new(btn_title)
            .min_size(vec2(RIGHT_WIDTH, HEIGHT))
            .stroke(Stroke::new(1.0, colors::BORDER))
            .fill(colors::WHITE)
            .corner_radius(corners::MEDIUM);

        let modal_id = ui.id().with("pnr_modal_state");
        let modal_is_open = ui.data(|d| d.get_temp::<bool>(modal_id).unwrap_or(false));

        if ui.add(btn).clicked() {
            ui.data_mut(|d| d.insert_temp(modal_id, true));
        }

        if modal_is_open {
            let close = Modal::new("pnr_modal")
                .width(200.0)
                .show(self.ctx, |ui| self.show_pnr_modal(ui));

            if close {
                ui.data_mut(|d| d.insert_temp(modal_id, false));
            };
        }
    }

    fn show_pnr_modal(&self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Uly adam").size(16.0).color(colors::BLACK));
        });

        ui.add_space(20.0);

        ui.columns(3, |columns| {
            let btn_minus = Button::new(RichText::new("-").size(28.0).color(colors::BLACK))
                .min_size(vec2(50.0, 50.0))
                .fill(colors::WHITE)
                .stroke(Stroke::new(1.0, colors::BORDER))
                .corner_radius(corners::SMALL);

            let btn_plus = Button::new(RichText::new("+").size(28.0).color(colors::BLACK))
                .min_size(vec2(50.0, 50.0))
                .fill(colors::WHITE)
                .stroke(Stroke::new(1.0, colors::BORDER))
                .corner_radius(corners::SMALL);

            columns[0].vertical_centered(|ui| {
                if ui.add(btn_minus).clicked() {
                    debug!("Minus clicked");
                }
            });

            columns[1].vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(RichText::new("9").size(24.0).color(colors::BLACK));
            });

            columns[2].vertical_centered(|ui| {
                if ui.add(btn_plus).clicked() {
                    debug!("Plus clicked");
                }
            });
        });
    }
}
