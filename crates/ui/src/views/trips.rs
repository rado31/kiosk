use std::sync::mpsc;

use egui::{
    Align, FontFamily, Frame, Image, Layout, Rect, RichText, ScrollArea, Sense, Spinner, Stroke,
    StrokeKind, Ui, UiBuilder, include_image, vec2,
};

use crate::{
    i18n::t,
    state::State,
    theme::{colors, corners},
    views::View,
};

pub fn show(state: &mut State, ctx: &egui::Context, ui: &mut Ui) {
    poll_trips(state, ctx);

    let (rect, res) = ui.allocate_exact_size(vec2(100.0, 60.0), Sense::CLICK);

    ui.painter().rect(
        rect,
        corners::MEDIUM,
        colors::BTN_PRIMARY_BG,
        Stroke::new(1.0, colors::BORDER),
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
        show_centered(ui, |ui| {
            ui.add(Spinner::new().size(48.0));
        });

        return;
    }

    if state.trips.has_error {
        show_centered(ui, |ui| {
            let msg = RichText::new(t(&state.lang, "trips_fetch_error"))
                .size(22.0)
                .color(colors::ERROR);
            ui.label(msg);
        });
        return;
    }

    let Some(trips) = state.trips.get_trips() else {
        return;
    };

    if trips.is_empty() {
        show_centered(ui, |ui| {
            let msg = RichText::new(t(&state.lang, "trips_not_found"))
                .size(22.0)
                .color(colors::FG_MUTED);
            ui.label(msg);
        });
        return;
    }

    let trips = trips.clone();

    let is_turkmen = state.lang.is_turkmen();
    let source = state
        .trips
        .get_source()
        .map_or("", |s| s.get_title(is_turkmen));

    let destination = state
        .trips
        .get_destination()
        .map_or("", |s| s.get_title(is_turkmen));

    let title_str = format!("source} - {destination}");
    let title = RichText::new(title_str)
        .size(28.0)
        .family(FontFamily::Name("bold".into()))
        .color(colors::BLACK);

    ui.vertical_centered(|ui| ui.label(title));
    ui.add_space(40.0);

    // Cards
    ScrollArea::vertical().show(ui, |ui| {
        for trip in &trips {
            render_trip_card(state, ui, trip);
            ui.add_space(12.0);
        }
    });
}

fn poll_trips(state: &mut State, ctx: &egui::Context) {
    let Some(rx) = state.trips.take_receiver() else {
        return;
    };

    match rx.try_recv() {
        Ok(data) => {
            state.trips.set_result(data);
            ctx.request_repaint();
        }
        Err(mpsc::TryRecvError::Empty) => {
            state.trips.start_fetching(rx);
        }
        Err(mpsc::TryRecvError::Disconnected) => {
            state.trips.set_result(None);
        }
    }
}

fn show_centered(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), ui.available_height()),
        Layout::centered_and_justified(egui::Direction::TopDown),
        |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(ui.available_height() / 3.0);
                content(ui);
            });
        },
    );
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

fn render_trip_card(state: &State, ui: &mut Ui, trip: &api::trips::Trip) {
    let card = Frame::new()
        .inner_margin(20.0)
        .fill(colors::CARD_BG)
        .corner_radius(corners::MEDIUM)
        .stroke(Stroke::new(1.0, colors::CARD_BORDER));

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
                    .color(colors::BLACK);

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
                    .color(colors::BLACK);

                ui.label(time);
                ui.add_space(10.0);
                ui.label(RichText::new(&arr_date).size(16.0).color(colors::FG_MUTED));
            });
        });

        ui.add_space(20.0);

        // Wagon types with prices
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

                let (bg, fg) = if disabled {
                    (colors::BG_5, colors::FG_DISABLED)
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
                    log::debug!("{} clicked", wt.wagon_type_id);
                }
            }
        });
    });
}
