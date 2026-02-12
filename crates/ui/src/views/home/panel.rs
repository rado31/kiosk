use std::{sync::mpsc, thread};

use egui::{
    Align, Align2, Button, Color32, FontFamily, FontId, Frame, Layout, Pos2, Rect, RichText, Sense,
    Shadow, Stroke, StrokeKind, Ui, pos2, vec2,
};

use egui_toast::{Toast, ToastKind, ToastOptions};

use crate::{
    components::modal::Modal,
    i18n::t,
    state::{State, modal::Modal as ModalKind, trips::TripKind},
    theme::{colors, corners},
    views::View,
};

pub fn top_left(state: &mut State, ui: &mut Ui) {
    let frame = Frame::new()
        .inner_margin(5)
        .corner_radius(corners::MEDIUM)
        .fill(colors::WHITE)
        .shadow(Shadow {
            offset: [0, 2],
            blur: 8,
            spread: 0,
            color: colors::SHADOW,
        });

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
        state.trips.kind = TripKind::OneWay;
    }

    if res2.clicked() {
        state.trips.kind = TripKind::Round;
    }

    let is_one_way = state.trips.kind == TripKind::OneWay;
    let curr_rect = if is_one_way { rect1 } else { rect2 };

    let anime_x =
        ui.ctx()
            .animate_value_with_time(ui.id().with("tab_indicator"), curr_rect.min.x, 0.2);
    let indicator_rect = Rect::from_min_size(pos2(anime_x, curr_rect.min.y), curr_rect.size());

    ui.painter()
        .rect_filled(indicator_rect, corners::MEDIUM, colors::BTN_PRIMARY_BG);

    let (txt_color1, txt_color2) = if is_one_way {
        (colors::WHITE, colors::FG)
    } else {
        (colors::FG, colors::WHITE)
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
    let total_pnrs = format!(
        "{}  {}",
        t(&state.lang, "pnrs_count"),
        state.passengers.total()
    );

    let (rect, res) = ui.allocate_exact_size(vec2(250.0, 60.0), Sense::CLICK);
    let shadow = Shadow {
        offset: [0, 2],
        blur: 8,
        spread: 0,
        color: colors::SHADOW,
    };

    ui.painter().add(shadow.as_shape(rect, corners::MEDIUM));
    ui.painter().rect(
        rect,
        corners::MEDIUM,
        colors::WHITE,
        Stroke::NONE,
        StrokeKind::Outside,
    );

    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        total_pnrs,
        FontId::proportional(16.0),
        colors::FG,
    );

    if res.clicked() {
        state.modal = ModalKind::PnrCounts;
    }

    if state.modal == ModalKind::PnrCounts {
        let should_close = Modal::new("pnr_counts_modal")
            .width(500.0)
            .open(ctx, |ui| render_pnr_counts_modal(state, ui));

        if should_close {
            state.modal = ModalKind::Closed;
        };
    }
}

fn render_pnr_counts_modal(state: &mut State, ui: &mut Ui) {
    render_pnr_row(state, ui, true);
    ui.add_space(16.0);
    render_pnr_row(state, ui, false);
    ui.add_space(24.0);
    ui.vertical_centered(|ui| {
        let lbl = RichText::new(t(&state.lang, "pnrs_max"))
            .size(16.0)
            .color(colors::FG_MUTED);

        ui.label(lbl);
    });
}

fn render_pnr_row(state: &mut State, ui: &mut Ui, is_adult: bool) {
    let card = Frame::new()
        .inner_margin(20.0)
        .fill(colors::WHITE)
        .corner_radius(corners::MEDIUM)
        .shadow(Shadow {
            offset: [0, 2],
            blur: 8,
            spread: 0,
            color: colors::SHADOW,
        });

    card.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.set_height(80.0);

        let title = if is_adult {
            t(&state.lang, "adult")
        } else {
            t(&state.lang, "child")
        };

        let count = if is_adult {
            state.passengers.adults
        } else {
            state.passengers.children
        };

        let can_sub = count > 0 && state.passengers.total() > 1;
        let can_add = state.passengers.total() < 9;

        ui.horizontal_centered(|ui| {
            ui.label(RichText::new(title).size(24.0).color(colors::FG));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let size = 60.0;
                let corner = 30.0;

                let (plus_bg, plus_fg) = if can_add {
                    (colors::PRIMARY, colors::WHITE)
                } else {
                    (colors::BG_5, colors::FG_DISABLED)
                };

                let (plus_rec, plus_res) = ui.allocate_exact_size(vec2(size, size), Sense::CLICK);

                ui.painter().rect_filled(plus_rec, corner, plus_bg);
                ui.painter().text(
                    plus_rec.center() - vec2(0.0, 2.0),
                    Align2::CENTER_CENTER,
                    "+",
                    FontId::new(30.0, FontFamily::Name("bold".into())),
                    plus_fg,
                );

                let (count_rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::empty());

                ui.painter().text(
                    count_rect.center(),
                    Align2::CENTER_CENTER,
                    format!("{count}"),
                    FontId::new(30.0, FontFamily::Name("bold".into())),
                    colors::FG,
                );

                let (minus_rect, minus_res) =
                    ui.allocate_exact_size(vec2(size, size), Sense::CLICK);

                let (minus_bg, minus_fg) = if can_sub {
                    (colors::PRIMARY, colors::WHITE)
                } else {
                    (colors::BG_5, colors::FG_DISABLED)
                };

                ui.painter().rect_filled(minus_rect, corner, minus_bg);
                ui.painter().text(
                    minus_rect.center() - vec2(0.0, 2.0),
                    Align2::CENTER_CENTER,
                    "-",
                    FontId::new(24.0, FontFamily::Name("bold".into())),
                    minus_fg,
                );

                if plus_res.clicked() && can_add {
                    if is_adult {
                        state.passengers.add_adult();
                    } else {
                        state.passengers.add_child();
                    }
                }

                if minus_res.clicked() && can_sub {
                    if is_adult {
                        state.passengers.remove_adult();
                    } else {
                        state.passengers.remove_child();
                    }
                }
            });
        });
    });
}

pub fn bottom(state: &mut State, ui: &mut Ui) {
    const BTN_HEIGHT: f32 = 60.0;
    const PADDING: f32 = 10.0;

    let shadow = Shadow {
        offset: [0, 2],
        blur: 8,
        spread: 0,
        color: colors::SHADOW,
    };

    let paint_col_btn = |ui: &mut Ui, text: &str| {
        let width = ui.available_width() - PADDING;
        let (rect, res) = ui.allocate_exact_size(vec2(width, BTN_HEIGHT), Sense::CLICK);

        ui.painter().add(shadow.as_shape(rect, corners::MEDIUM));
        ui.painter()
            .rect_filled(rect, corners::MEDIUM, colors::WHITE);

        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            text,
            FontId::proportional(18.0),
            colors::FG,
        );

        res
    };

    ui.columns_const(|[col1, col2, col3, col4]| {
        let is_turkmen = state.lang.is_turkmen();

        col1.vertical_centered(|ui| {
            let source_label = match &state.trips.source {
                Some(s) => {
                    if is_turkmen {
                        s.title_tm.as_str()
                    } else {
                        s.title_ru.as_str()
                    }
                }
                None => t(&state.lang, "from"),
            };

            if paint_col_btn(ui, source_label).clicked() {
                state.modal = ModalKind::Source;
            }
        });

        col2.vertical_centered(|ui| {
            let dest_label = match &state.trips.destination {
                Some(s) => {
                    if is_turkmen {
                        s.title_tm.as_str()
                    } else {
                        s.title_ru.as_str()
                    }
                }
                None => t(&state.lang, "to"),
            };

            if paint_col_btn(ui, dest_label).clicked() {
                state.modal = ModalKind::Destination;
            }
        });

        col3.vertical_centered(|ui| {
            let ow = state.calendar.one_way_date;
            let ow_label = ow.format("%d.%m.%Y").to_string();

            if paint_col_btn(ui, &ow_label).clicked() {
                let date = state.calendar.one_way_date;
                state.calendar.view_date(date);
                state.modal = ModalKind::OneWayCalendar;
            }
        });

        if state.trips.kind != TripKind::OneWay {
            col4.vertical_centered(|ui| {
                let rt = state.calendar.round_trip_date;
                let rt_label = rt.format("%d.%m.%Y").to_string();

                if paint_col_btn(ui, &rt_label).clicked() {
                    let date = state.calendar.round_trip_date;
                    state.calendar.view_date(date);
                    state.modal = ModalKind::RoundTripCalendar;
                }
            });
        }
    });

    ui.add_space(20.0);

    let is_cooldown = state.trips.search_on_cooldown();

    let (bg, fg) = if is_cooldown {
        (colors::BG_5, colors::FG_DISABLED)
    } else {
        (colors::BTN_PRIMARY_BG, colors::WHITE)
    };

    let search_btn = Button::new(RichText::new(t(&state.lang, "search")).size(18.0).color(fg))
        .min_size(vec2(150.0, BTN_HEIGHT))
        .fill(bg)
        .corner_radius(corners::MEDIUM);

    ui.vertical_centered(|ui| {
        if ui.add(search_btn).clicked() && !is_cooldown {
            state.trips.mark_searched();

            let Some(source) = &state.trips.source else {
                state.toasts.add(Toast {
                    text: t(&state.lang, "select_source").into(),
                    kind: ToastKind::Error,
                    options: ToastOptions::default().duration_in_seconds(3.0),
                    ..Default::default()
                });

                return;
            };

            let Some(destination) = &state.trips.destination else {
                state.toasts.add(Toast {
                    text: t(&state.lang, "select_destination").into(),
                    kind: ToastKind::Error,
                    options: ToastOptions::default().duration_in_seconds(3.0),
                    ..Default::default()
                });

                return;
            };

            let source_id = source.id;
            let destination_id = destination.id;
            let date = state.calendar.one_way_date.format("%Y-%m-%d").to_string();
            let adult = state.passengers.adults as u32;
            let child = state.passengers.children as u32;
            let is_round = state.trips.kind == TripKind::Round;

            let inbound_date = if is_round {
                Some(
                    state
                        .calendar
                        .round_trip_date
                        .format("%Y-%m-%d")
                        .to_string(),
                )
            } else {
                None
            };

            let (tx, rx) = mpsc::channel();
            state.trips.start_fetching(rx);
            state.go_to(View::Trips);

            thread::spawn(move || {
                let outbound = match api::trips::fetch(api::trips::TripsParams {
                    source: source_id,
                    destination: destination_id,
                    date: &date,
                    adult,
                    child,
                }) {
                    Ok(trips) => Some(trips),
                    Err(e) => {
                        log::error!("Error on `GET /trips` (outbound). {e}");
                        None
                    }
                };

                let inbound = if let Some(rd) = &inbound_date {
                    match api::trips::fetch(api::trips::TripsParams {
                        source: destination_id,
                        destination: source_id,
                        date: rd,
                        adult,
                        child,
                    }) {
                        Ok(trips) => Some(trips),
                        Err(e) => {
                            log::error!("Error on `GET /trips` (inbound). {e}");
                            None
                        }
                    }
                } else {
                    None
                };

                tx.send((outbound, inbound)).ok();
            });
        }
    });
}
