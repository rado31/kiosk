use std::{sync::mpsc, thread};

use egui::{
    Align, FontFamily, Frame, Image, Layout, Rect, RichText, ScrollArea, Sense, Shadow, Spinner,
    Stroke, StrokeKind, Ui, UiBuilder, include_image, vec2,
};

use crate::{
    i18n::t,
    state::{State, trips::TripKind},
    theme::{colors, corners},
    views::View,
};

#[derive(Clone, Copy, PartialEq)]
enum TripSection {
    Outbound,
    Inbound,
}

fn poll_trips(state: &mut State, ctx: &egui::Context) {
    let Some(rx) = state.trips.take_receiver() else {
        return;
    };

    match rx.try_recv() {
        Ok((outbound, inbound)) => {
            state.trips.set_result(outbound, inbound);
            ctx.request_repaint();
        }
        Err(mpsc::TryRecvError::Empty) => {
            state.trips.start_fetching(rx);
            ctx.request_repaint();
        }
        Err(mpsc::TryRecvError::Disconnected) => {
            state.trips.set_result(None, None);
        }
    }
}

pub fn show(state: &mut State, ctx: &egui::Context, ui: &mut Ui) {
    poll_trips(state, ctx);

    let (rect, res) = ui.allocate_exact_size(vec2(100.0, 60.0), Sense::CLICK);
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
        colors::BTN_PRIMARY_BG,
        Stroke::NONE,
        StrokeKind::Outside,
    );

    let img =
        Image::new(include_image!("../assets/left_arrow.svg")).fit_to_exact_size(vec2(30.0, 30.0));

    let img_rect = Rect::from_center_size(rect.center(), vec2(30.0, 30.0));
    let mut img_ui = ui.new_child(UiBuilder::new().max_rect(img_rect));

    img_ui.add(img);

    if res.clicked() {
        state.go_to(View::Home);
    }

    ui.add_space(20.0);

    if state.trips.is_fetching() {
        ui.centered_and_justified(|ui| ui.add(Spinner::new().size(50.0).color(colors::PRIMARY)));
        return;
    }

    let is_round = state.trips.kind == TripKind::Round;
    let is_turkmen = state.lang.is_turkmen();

    let source_name = state
        .trips
        .get_source()
        .map_or_else(String::new, |s| s.get_title(is_turkmen).to_string());

    let dest_name = state
        .trips
        .get_destination()
        .map_or_else(String::new, |s| s.get_title(is_turkmen).to_string());

    let outbound_title = format!("{source_name} - {dest_name}");
    let inbound_title = format!("{dest_name} - {source_name}");
    let outbound_has_error = state.trips.outbound_has_error;
    let inbound_has_error = state.trips.inbound_has_error;
    // Clone to release the borrow on state before passing state mutably into the scroll closure.
    let outbound = state.trips.get_outbound().cloned();
    let inbound = state.trips.get_inbound().cloned();

    ScrollArea::vertical().show(ui, |ui| {
        render_trip_section(
            state,
            ui,
            &outbound_title,
            outbound_has_error,
            outbound.as_deref(),
            is_round,
            TripSection::Outbound,
        );

        if is_round {
            ui.add_space(40.0);

            render_trip_section(
                state,
                ui,
                &inbound_title,
                inbound_has_error,
                inbound.as_deref(),
                is_round,
                TripSection::Inbound,
            );
        }
    });
}

fn render_trip_section(
    state: &mut State,
    ui: &mut Ui,
    title_str: &str,
    has_error: bool,
    trips: Option<&[api::trips::Trip]>,
    is_round: bool,
    section: TripSection,
) {
    let title = RichText::new(title_str)
        .size(28.0)
        .family(FontFamily::Name("bold".into()))
        .color(colors::FG);

    ui.vertical_centered(|ui| ui.label(title));
    ui.add_space(20.0);

    if has_error {
        let msg = RichText::new(t(&state.lang, "trips_fetch_error"))
            .size(22.0)
            .color(colors::ERROR);

        ui.vertical_centered(|ui| ui.label(msg));
        return;
    }

    let Some(trips) = trips else {
        return;
    };

    if trips.is_empty() {
        let msg = RichText::new(t(&state.lang, "trips_not_found"))
            .size(22.0)
            .color(colors::FG_MUTED);

        ui.vertical_centered(|ui| ui.label(msg));
        return;
    }

    for trip in trips {
        render_trip_card(state, ui, trip, is_round, section);
        ui.add_space(12.0);
    }
}

/// Parses `2026-02-20T20:30:00+05:00` into `("20:30", "20.02.2026")`.
fn parse_datetime(datetime: &str) -> (String, String) {
    let (date_part, rest) = datetime.split_once('T').unwrap_or((datetime, ""));
    let time = rest.get(..5).unwrap_or(rest);
    let date: Vec<_> = date_part.split('-').collect();
    let date = match date[..] {
        [y, m, d] => format!("{d}.{m}.{y}"),
        _ => date_part.to_string(),
    };

    (time.to_string(), date)
}

fn render_trip_card(
    state: &mut State,
    ui: &mut Ui,
    trip: &api::trips::Trip,
    is_round: bool,
    section: TripSection,
) {
    let card = Frame::new()
        .inner_margin(24.0)
        .fill(colors::CARD_BG)
        .corner_radius(corners::MEDIUM)
        .shadow(Shadow {
            offset: [0, 2],
            blur: 8,
            spread: 0,
            color: colors::SHADOW,
        });

    card.show(ui, |ui| {
        ui.set_width(ui.available_width());

        let (dep_time, dep_date) = parse_datetime(&trip.departure_time);
        let (arr_time, arr_date) = parse_datetime(&trip.arrival_time);

        ui.columns_const(|[col1, col2, col3, col4]| {
            col1.vertical_centered(|ui| {
                let label = RichText::new(t(&state.lang, "departure"))
                    .size(18.0)
                    .color(colors::FG_MUTED);

                ui.label(label);
                ui.add_space(10.0);

                let time = RichText::new(&dep_time)
                    .size(28.0)
                    .family(FontFamily::Name("bold".into()))
                    .color(colors::FG);

                ui.label(time);
                ui.add_space(10.0);

                ui.label(RichText::new(&dep_date).size(16.0).color(colors::FG_MUTED));
            });

            col2.vertical_centered(|ui| {
                let icon = Image::new(include_image!("../assets/time.svg"))
                    .fit_to_exact_size(vec2(50.0, 50.0))
                    .tint(colors::FG_MUTED);

                ui.add_space(10.0);
                ui.add(icon);
                ui.add_space(10.0);

                let hours = trip.travel_time / 60;
                let mins = trip.travel_time % 60;
                let travel_str = format!(
                    "{} {} {} {}",
                    hours,
                    t(&state.lang, "hour_short"),
                    mins,
                    t(&state.lang, "min_short"),
                );

                ui.label(RichText::new(travel_str).size(18.0).color(colors::FG_MUTED));
            });

            col3.vertical_centered(|ui| {
                let icon = Image::new(include_image!("../assets/distance.svg"))
                    .fit_to_exact_size(vec2(50.0, 50.0))
                    .tint(colors::FG_MUTED);

                ui.add_space(10.0);
                ui.add(icon);
                ui.add_space(10.0);

                let dist_str = format!("{} {}", trip.distance, t(&state.lang, "km"));
                ui.label(RichText::new(dist_str).size(18.0).color(colors::FG_MUTED));
            });

            col4.vertical_centered(|ui| {
                let label = RichText::new(t(&state.lang, "arrival"))
                    .size(18.0)
                    .color(colors::FG_MUTED);

                ui.label(label);
                ui.add_space(10.0);

                let time = RichText::new(&arr_time)
                    .size(28.0)
                    .family(FontFamily::Name("bold".into()))
                    .color(colors::FG);

                ui.label(time);
                ui.add_space(10.0);
                ui.label(RichText::new(&arr_date).size(16.0).color(colors::FG_MUTED));
            });
        });

        ui.add_space(20.0);

        let wagon_types = &trip.wagon_types;

        ui.columns_const(|cols: &mut [Ui; 4]| {
            for (i, wt) in wagon_types.iter().enumerate() {
                let col = &mut cols[i];
                let width = col.available_width() - 8.0;
                let disabled = !wt.has_seats;

                let sense = if disabled {
                    Sense::empty()
                } else {
                    Sense::CLICK
                };

                let (rect, res) = col.allocate_exact_size(vec2(width, 60.0), sense);

                let is_selected = is_round
                    && match section {
                        TripSection::Outbound => state
                            .trips
                            .outbound_selection
                            .is_some_and(|(tid, wid)| tid == trip.id && wid == wt.wagon_type_id),
                        TripSection::Inbound => state
                            .trips
                            .inbound_selection
                            .is_some_and(|(tid, wid)| tid == trip.id && wid == wt.wagon_type_id),
                    };

                let (bg, fg) = if disabled {
                    (colors::BG_5, colors::FG_DISABLED)
                } else if is_selected {
                    (colors::PRIMARY, colors::WHITE)
                } else {
                    (colors::PRIMARY_BG, colors::PRIMARY)
                };

                col.painter()
                    .rect(rect, corners::MEDIUM, bg, Stroke::NONE, StrokeKind::Outside);

                let inner = rect.shrink(18.0);
                let mut child = col.new_child(UiBuilder::new().max_rect(inner));

                child.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        let title = RichText::new(&wt.wagon_type_title).size(18.0).color(fg);

                        ui.label(title);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let price = RichText::new(format!("{:.2} TMT", wt.price))
                                .size(18.0)
                                .family(FontFamily::Name("bold".into()))
                                .color(fg);

                            ui.label(price);
                        });
                    });
                });

                if !disabled && res.clicked() {
                    let trip_id = trip.id;
                    let adult = state.passengers.adults as u8;
                    let child = state.passengers.children as u8;
                    let wagon_type_id = wt.wagon_type_id;
                    let passenger_count =
                        (state.passengers.adults + state.passengers.children) as usize;

                    let spawn_fetch = |trip_id: u32, wt_id: u32, tx: mpsc::Sender<_>| {
                        thread::spawn(move || {
                            let result =
                                match api::trips::fetch_details(api::trips::DetailsParams {
                                    trip_id,
                                    adult,
                                    child,
                                    wagon_type_id: wt_id,
                                }) {
                                    Ok(v) => Some(v),
                                    Err(e) => {
                                        log::error!("{e}");
                                        None
                                    }
                                };

                            tx.send(result).ok();
                        });
                    };

                    if !is_round {
                        let (tx, rx) = mpsc::channel();
                        state.seats.init(passenger_count);
                        state.seats.start_fetching(rx);
                        state.go_to(View::Seats);
                        spawn_fetch(trip_id, wagon_type_id, tx);
                    } else {
                        match section {
                            TripSection::Outbound => {
                                state.trips.outbound_selection = Some((trip_id, wagon_type_id));
                            }
                            TripSection::Inbound => {
                                state.trips.inbound_selection = Some((trip_id, wagon_type_id));
                            }
                        }

                        if let (Some((out_trip_id, out_wt_id)), Some((in_trip_id, in_wt_id))) = (
                            state.trips.outbound_selection,
                            state.trips.inbound_selection,
                        ) {
                            let (tx_out, rx_out) = mpsc::channel();
                            let (tx_in, rx_in) = mpsc::channel();

                            state.seats.init(passenger_count);
                            state.seats.start_fetching(rx_out);
                            state.seats.start_fetching_inbound(rx_in);
                            state.go_to(View::Seats);

                            spawn_fetch(out_trip_id, out_wt_id, tx_out);
                            spawn_fetch(in_trip_id, in_wt_id, tx_in);
                        }
                    }
                }
            }
        });
    });
}
