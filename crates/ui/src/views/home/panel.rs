use egui::{
    Align2, Button, Color32, FontId, Frame, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, Ui,
    pos2, vec2,
};

use crate::{
    components::modal::Modal,
    i18n::t,
    state::{State, modal::Modal as ModalKind, trip::TripKind},
    theme::{colors, corners},
};

pub fn top_left(state: &mut State, ui: &mut Ui) {
    let frame = Frame::new()
        .inner_margin(5)
        .corner_radius(8)
        .stroke(Stroke::new(1.0, colors::BORDER));

    frame.show(ui, |ui| {
        ui.horizontal(|ui| render_trip_type_toggle(state, ui));
    });
}

fn render_trip_type_toggle(state: &mut State, ui: &mut Ui) {
    let btn_width: f32 = ui.available_width() / 2.0;
    const BTN_HEIGHT: f32 = 50.0;

    let (rect1, res1) = ui.allocate_exact_size(vec2(btn_width, BTN_HEIGHT), Sense::CLICK);
    let (rect2, res2) = ui.allocate_exact_size(vec2(btn_width, BTN_HEIGHT), Sense::CLICK);

    if res1.clicked() {
        state.trip.kind = TripKind::OneWay;
    }

    if res2.clicked() {
        state.trip.kind = TripKind::Round;
    }

    let is_one_way = state.trip.kind == TripKind::OneWay;
    let curr_rect = if is_one_way { rect1 } else { rect2 };

    let anime_x =
        ui.ctx()
            .animate_value_with_time(ui.id().with("tab_indicator"), curr_rect.min.x, 0.2);
    let indicator_rect = Rect::from_min_size(pos2(anime_x, curr_rect.min.y), curr_rect.size());

    ui.painter()
        .rect_filled(indicator_rect, corners::MEDIUM, colors::BTN_PRIMARY_BG);

    let (txt_color1, txt_color2) = if is_one_way {
        (colors::WHITE, colors::BLACK)
    } else {
        (colors::BLACK, colors::WHITE)
    };

    draw_centered_text(ui, rect1.center(), txt_color1, t(&state.lang, "one_way"));
    draw_centered_text(ui, rect2.center(), txt_color2, t(&state.lang, "round_trip"));
}

fn draw_centered_text(ui: &Ui, pos: Pos2, color: Color32, text: &str) {
    ui.painter().text(
        pos,
        Align2::CENTER_CENTER,
        text,
        FontId::proportional(16.0),
        color,
    );
}

pub fn top_right(state: &mut State, ctx: &egui::Context, ui: &mut Ui) {
    let total_pnrs = format!("{}  {}", t(&state.lang, "pnr"), state.passengers.total());

    let (rect, res) = ui.allocate_exact_size(vec2(120.0, 60.0), Sense::CLICK);

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
        FontId::proportional(16.0),
        colors::BLACK,
    );

    if res.clicked() {
        state.modal = ModalKind::PnrCounts;
    }

    if state.modal == ModalKind::PnrCounts {
        let should_close = Modal::new("pnr_counts_modal")
            .width(400.0)
            .open(ctx, |ui| render_pnr_counts_modal(state, ui));

        if should_close {
            state.modal = ModalKind::Closed;
        };
    }
}

fn render_pnr_counts_modal(state: &mut State, ui: &mut Ui) {
    render_pnr_counter(state, ui, true);
    ui.add_space(40.0);
    render_pnr_counter(state, ui, false);
}

fn render_pnr_counter(state: &mut State, ui: &mut Ui, is_adult: bool) {
    ui.vertical_centered(|ui| {
        let title = if is_adult {
            t(&state.lang, "adult")
        } else {
            t(&state.lang, "child")
        };

        ui.label(RichText::new(title).size(28.0).color(colors::BLACK));
    });

    ui.add_space(40.0);

    ui.columns_const(|[col1, col2, col3]| {
        let create_btn = |text: &str| {
            Button::new(RichText::new(text).size(36.0).color(colors::BLACK))
                .min_size(vec2(100.0, 100.0))
                .fill(colors::WHITE)
                .stroke(Stroke::new(1.0, colors::BORDER))
                .corner_radius(corners::SMALL)
        };

        col1.vertical_centered(|ui| {
            if ui.add(create_btn("-")).clicked() {
                if is_adult {
                    state.passengers.remove_adult();
                } else {
                    state.passengers.remove_child();
                }
            }
        });

        col2.vertical_centered(|ui| {
            ui.add_space(30.0);

            let count = if is_adult {
                format!("{}", state.passengers.adults)
            } else {
                format!("{}", state.passengers.children)
            };

            ui.label(RichText::new(count).size(36.0).color(colors::BLACK));
        });

        col3.vertical_centered(|ui| {
            if ui.add(create_btn("+")).clicked() {
                if is_adult {
                    state.passengers.add_adult();
                } else {
                    state.passengers.add_child();
                }
            }
        });
    });
}

pub fn bottom(state: &mut State, ui: &mut Ui) {
    const BTN_HEIGHT: f32 = 60.0;
    const PADDING: f32 = 10.0;

    let create_col_btn = |ui: &mut Ui, text: &str| {
        let label = RichText::new(text).size(18.0).color(colors::BLACK);
        let width = ui.available_width() - PADDING;

        Button::new(label)
            .min_size(vec2(width, BTN_HEIGHT))
            .stroke(Stroke::new(1.0, colors::BORDER))
            .fill(colors::WHITE)
            .corner_radius(corners::MEDIUM)
    };

    ui.columns_const(|[col1, col2, col3, col4]| {
        let is_turkmen = state.lang.is_turkmen();

        col1.vertical_centered(|ui| {
            let source_label = match &state.trip.source {
                Some(s) => {
                    if is_turkmen {
                        s.title_tm.as_str()
                    } else {
                        s.title_ru.as_str()
                    }
                }
                None => t(&state.lang, "from"),
            };

            let source_btn = create_col_btn(ui, source_label);

            if ui.add(source_btn).clicked() {
                state.modal = ModalKind::Source;
            }
        });

        col2.vertical_centered(|ui| {
            let dest_label = match &state.trip.destination {
                Some(s) => {
                    if is_turkmen {
                        s.title_tm.as_str()
                    } else {
                        s.title_ru.as_str()
                    }
                }
                None => t(&state.lang, "to"),
            };

            let destination_btn = create_col_btn(ui, dest_label);

            if ui.add(destination_btn).clicked() {
                state.modal = ModalKind::Destination;
            }
        });

        col3.vertical_centered(|ui| {
            let ow = state.calendar.one_way_date();
            let ow_label = ow.format("%d.%m.%Y").to_string();
            let one_way_btn = create_col_btn(ui, &ow_label);

            if ui.add(one_way_btn).clicked() {
                let date = state.calendar.one_way_date();
                state.calendar.view_date(date);
                state.modal = ModalKind::OneWayCalendar;
            }
        });

        if state.trip.kind != TripKind::OneWay {
            col4.vertical_centered(|ui| {
                let rt = state.calendar.round_trip_date;
                let rt_label = rt.format("%d.%m.%Y").to_string();
                let round_trip_btn = create_col_btn(ui, &rt_label);

                if ui.add(round_trip_btn).clicked() {
                    let date = state.calendar.round_trip_date;
                    state.calendar.view_date(date);
                    state.modal = ModalKind::RoundTripCalendar;
                }
            });
        }
    });

    ui.add_space(20.0);

    let search_lbl = RichText::new("Gozle").size(18.0).color(colors::WHITE);
    let search_btn = Button::new(search_lbl)
        .min_size(vec2(150.0, BTN_HEIGHT))
        .stroke(Stroke::new(1.0, colors::BORDER))
        .fill(colors::BTN_PRIMARY_BG)
        .corner_radius(corners::MEDIUM);

    ui.vertical_centered(|ui| {
        if ui.add(search_btn).clicked() {
            log::debug!("search button clicked");
        }
    });
}
