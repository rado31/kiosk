use egui::{
    Align2, Button, Color32, FontId, Frame, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, Ui,
    pos2, vec2,
};

use crate::{
    app::{
        components::base::Modal,
        constants::{colors, corners},
        i18n::t,
    },
    debug,
};

use super::Home;

impl<'a> Home<'a> {
    pub fn show_panel_top_left(&mut self, ui: &mut Ui) {
        let frame = Frame::new()
            .inner_margin(5)
            .corner_radius(8)
            .stroke(Stroke::new(1.0, colors::BORDER));

        frame.show(ui, |ui| {
            ui.horizontal(|ui| self.render_trip_type_toggle(ui));
        });
    }

    fn render_trip_type_toggle(&mut self, ui: &mut Ui) {
        let btn_width: f32 = ui.available_width() / 2.0;
        const BTN_HEIGHT: f32 = 40.0;

        let (rect1, res1) = ui.allocate_exact_size(vec2(btn_width, BTN_HEIGHT), Sense::CLICK);
        let (rect2, res2) = ui.allocate_exact_size(vec2(btn_width, BTN_HEIGHT), Sense::CLICK);

        if res1.clicked() {
            self.state.trip.set_one_way();
        }

        if res2.clicked() {
            self.state.trip.set_round();
        }

        let curr_rect = if self.state.trip.is_one_way() {
            rect1
        } else {
            rect2
        };

        let anime_x =
            ui.ctx()
                .animate_value_with_time(ui.id().with("tab_indicator"), curr_rect.min.x, 0.2);
        let indicator_rect = Rect::from_min_size(pos2(anime_x, curr_rect.min.y), curr_rect.size());

        ui.painter()
            .rect_filled(indicator_rect, corners::MEDIUM, colors::BTN_PRIMARY_BG);

        let (txt_color1, txt_color2) = if self.state.trip.is_one_way() {
            (colors::WHITE, colors::BLACK)
        } else {
            (colors::BLACK, colors::WHITE)
        };

        self.draw_centered_text(
            ui,
            rect1.center(),
            txt_color1,
            t(self.state.lang, "one_way"),
        );

        self.draw_centered_text(
            ui,
            rect2.center(),
            txt_color2,
            t(self.state.lang, "round_trip"),
        );
    }

    fn draw_centered_text(&mut self, ui: &Ui, pos: Pos2, color: Color32, text: &str) {
        ui.painter().text(
            pos,
            Align2::CENTER_CENTER,
            text,
            FontId::proportional(14.0),
            color,
        );
    }

    pub fn show_panel_top_right(&mut self, ui: &mut Ui) {
        let total_pnrs = format!(
            "{}  {}",
            t(self.state.lang, "pnr"),
            self.state.pnr_counts.total()
        );

        let (rect, res) = ui.allocate_exact_size(vec2(100.0, 50.0), Sense::CLICK);

        ui.painter().rect(
            rect,
            corners::MEDIUM,
            colors::WHITE,
            Stroke::new(1.0, colors::BORDER),
            StrokeKind::Outside,
        );

        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            total_pnrs,
            FontId::proportional(14.0),
            colors::BLACK,
        );

        if res.clicked() {
            self.state.open_pnr_counts_modal();
        }

        if self.state.is_pnr_counts_modal_opened() {
            let should_close = Modal::new("pnr_counts_modal")
                .width(200.0)
                .open(self.ctx, |ui| self.show_pnr_counts_modal(ui));

            if should_close {
                self.state.close_modal();
            };
        }
    }

    fn show_pnr_counts_modal(&mut self, ui: &mut Ui) {
        self.render_pnr_counter(ui, true);
        ui.add_space(20.0);
        self.render_pnr_counter(ui, false);
    }

    fn render_pnr_counter(&mut self, ui: &mut Ui, is_adult: bool) {
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

            col1.vertical_centered(|ui| {
                if ui.add(create_btn("-")).clicked() {
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
                if ui.add(create_btn("+")).clicked() {
                    if is_adult {
                        self.state.pnr_counts.add_adult();
                    } else {
                        self.state.pnr_counts.add_child();
                    }
                }
            });
        });
    }

    pub fn show_panel_bottom(&mut self, ui: &mut Ui) {
        const BTN_HEIGHT: f32 = 50.0;
        const PADDING: f32 = 10.0;

        let create_col_btn = |ui: &mut Ui, text: &str| {
            let label = RichText::new(text).size(14.0).color(colors::BLACK);
            let width = ui.available_width() - PADDING;

            Button::new(label)
                .min_size(vec2(width, BTN_HEIGHT))
                .stroke(Stroke::new(1.0, colors::BORDER))
                .fill(colors::WHITE)
                .corner_radius(corners::MEDIUM)
        };

        ui.columns_const(|[col1, col2, col3, col4]| {
            col1.vertical_centered(|ui| {
                let source = self.state.trip.get_source();
                let source = if self.state.is_turkmen_lang() {
                    source.title_tm
                } else {
                    source.title_ru
                };

                let source_btn = create_col_btn(ui, &source);

                if ui.add(source_btn).clicked() {
                    self.state.open_stations_modal()
                }
            });

            col2.vertical_centered(|ui| {
                let destination_btn = create_col_btn(ui, "destination");

                if ui.add(destination_btn).clicked() {
                    self.state.open_stations_modal()
                }
            });

            col3.vertical_centered(|ui| {
                let one_way_btn = create_col_btn(ui, "one way");

                if ui.add(one_way_btn).clicked() {
                    debug!("one way clicked");
                }
            });

            if !self.state.trip.is_one_way() {
                col4.vertical_centered(|ui| {
                    let round_trip_btn = create_col_btn(ui, "round_trip");

                    if ui.add(round_trip_btn).clicked() {
                        debug!("round trip clicked");
                    }
                });
            }
        });

        ui.add_space(20.0);

        let search_lbl = RichText::new("Gozle").size(14.0).color(colors::WHITE);
        let search_btn = Button::new(search_lbl)
            .min_size(vec2(150.0, BTN_HEIGHT))
            .stroke(Stroke::new(1.0, colors::BORDER))
            .fill(colors::BTN_PRIMARY_BG)
            .corner_radius(corners::MEDIUM);

        ui.vertical_centered(|ui| {
            if ui.add(search_btn).clicked() {
                debug!("search button clicked");
            }
        });
    }
}
