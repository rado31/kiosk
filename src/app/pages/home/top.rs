use super::*;
use crate::debug;

pub const LEFT_WIDTH: f32 = 300.0;
pub const RIGHT_WIDTH: f32 = 100.0;
const HEIGHT: f32 = 40.0;

impl<'a> Home<'a> {
    pub fn show_top_left(&mut self, ui: &mut Ui) {
        let frame = Frame::new()
            .inner_margin(5)
            .corner_radius(8)
            .stroke(Stroke::new(1.0, colors::BORDER));

        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                let (rect1, res1) =
                    ui.allocate_exact_size(vec2(LEFT_WIDTH / 2.0, HEIGHT), Sense::CLICK);

                let (rect2, res2) =
                    ui.allocate_exact_size(vec2(LEFT_WIDTH / 2.0, HEIGHT), Sense::CLICK);

                if res1.clicked() {
                    self.state.set_one_way_trip();
                }

                if res2.clicked() {
                    self.state.set_round_trip();
                }

                let curr_rect = if self.state.is_one_way_trip() {
                    rect1
                } else {
                    rect2
                };

                let curr_x = ui.ctx().animate_value_with_time(
                    ui.id().with("tab_indicator"),
                    curr_rect.min.x,
                    0.2,
                );

                let anime_rect = Rect::from_min_size(
                    pos2(curr_x, curr_rect.min.y),
                    vec2(curr_rect.width(), curr_rect.height()),
                );

                ui.painter()
                    .rect_filled(anime_rect, corners::MEDIUM, colors::BTN_PRIMARY_BG);

                let (txt_col1, txt_col2) = if self.state.is_one_way_trip() {
                    (colors::WHITE, colors::BLACK)
                } else {
                    (colors::BLACK, colors::WHITE)
                };

                let write_lbl = |pos: Pos2, color: Color32, label: &str| {
                    ui.painter().text(
                        pos,
                        Align2::CENTER_CENTER,
                        label,
                        FontId::proportional(14.0),
                        color,
                    );
                };

                write_lbl(rect1.center(), txt_col1, t(self.state.lang, "one_way"));
                write_lbl(rect2.center(), txt_col2, t(self.state.lang, "round_trip"));
            });
        });
    }

    pub fn show_top_right(&mut self, ui: &mut Ui) {
        let total_pnrs = format!(
            "{}  {}",
            t(self.state.lang, "pnr"),
            self.state.pnr_counts.total()
        );

        let btn_title = RichText::new(total_pnrs).size(14.0).color(colors::BLACK);
        let btn = Button::new(btn_title)
            .min_size(vec2(RIGHT_WIDTH, HEIGHT))
            .stroke(Stroke::new(1.0, colors::BORDER))
            .fill(colors::WHITE)
            .corner_radius(corners::MEDIUM);

        let mdl_id = ui.id().with("pnr_modal_state");
        let mdl_is_open = ui.data(|d| d.get_temp::<bool>(mdl_id).unwrap_or(false));

        if ui.add(btn).clicked() {
            ui.data_mut(|d| d.insert_temp(mdl_id, true));
        }

        if mdl_is_open {
            let close = Modal::new("pnr_modal")
                .width(200.0)
                .show(self.ctx, |ui| self.show_pnr_modal(ui));

            if close {
                ui.data_mut(|d| d.insert_temp(mdl_id, false));
            };
        }
    }

    fn show_pnr_modal(&mut self, ui: &mut Ui) {
        self.show_pnr_type_count(ui, true);
        ui.add_space(20.0);
        self.show_pnr_type_count(ui, false);
    }

    fn show_pnr_type_count(&mut self, ui: &mut Ui, is_adult: bool) {
        ui.vertical_centered(|ui| {
            let title = if is_adult {
                t(self.state.lang, "adult")
            } else {
                t(self.state.lang, "child")
            };

            ui.label(RichText::new(title).size(16.0).color(colors::BLACK));
        });

        ui.add_space(20.0);

        ui.columns_const(|[col1, col2, col3]| {
            let create_btn = |text: &str| {
                Button::new(RichText::new(text).size(28.0).color(colors::BLACK))
                    .min_size(vec2(50.0, 50.0))
                    .fill(colors::WHITE)
                    .stroke(Stroke::new(1.0, colors::BORDER))
                    .corner_radius(corners::SMALL)
            };

            let btn_minus = create_btn("-");
            let btn_plus = create_btn("+");

            col1.vertical_centered(|ui| {
                if ui.add(btn_minus).clicked() {
                    if is_adult {
                        self.state.pnr_counts.remove_adult();
                    } else {
                        self.state.pnr_counts.remove_child();
                    }
                }
            });

            col2.vertical_centered(|ui| {
                ui.add_space(10.0);

                let count = if is_adult {
                    format!("{}", self.state.pnr_counts.adults)
                } else {
                    format!("{}", self.state.pnr_counts.children)
                };

                ui.label(RichText::new(count).size(24.0).color(colors::BLACK));
            });

            col3.vertical_centered(|ui| {
                if ui.add(btn_plus).clicked() {
                    if is_adult {
                        self.state.pnr_counts.add_adult();
                    } else {
                        self.state.pnr_counts.add_child();
                    }
                }
            });
        });
    }
}
