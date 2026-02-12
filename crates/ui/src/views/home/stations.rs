use egui::{
    Align2, Context, FontFamily, FontId, Frame, RichText, ScrollArea, Sense, Shadow, Ui, vec2,
};

use crate::{
    components::modal::Modal,
    i18n::t,
    state::{State, modal::Modal as ModalKind},
    theme::{alphabet, colors, corners},
};
use core::config::POPULAR_STATION_IDS;

pub fn show(state: &mut State, ctx: &Context) {
    if state.modal == ModalKind::Source || state.modal == ModalKind::Destination {
        let should_close = Modal::new("stations_modal").width(880.0).open(ctx, |ui| {
            render_popular(state, ui);

            ui.add_space(40.0);

            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(t(&state.lang, "find_by_letters"))
                        .size(24.0)
                        .family(FontFamily::Name("bold".into()))
                        .color(colors::FG),
                );
            });

            ui.add_space(20.0);

            ui.columns_const(|[col1, col2]| {
                col1.vertical(|ui| render_letters(state, ui));
                col2.vertical(|ui| render_stations(state, ui));
            });
        });

        if should_close || state.trips.selected {
            state.modal = ModalKind::Closed;
            state.trips.selected = false;
        };
    }
}

fn render_popular(state: &mut State, ui: &mut Ui) {
    let Some(stations) = state.stations.get() else {
        return;
    };

    let is_source_modal = state.modal == ModalKind::Source;
    let is_turkmen = state.lang.is_turkmen();
    let selected_id = station_id_for(state, is_source_modal);
    let other_id = station_id_for(state, !is_source_modal);

    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new(t(&state.lang, "most_popular_places"))
                .size(24.0)
                .family(FontFamily::Name("bold".into()))
                .color(colors::FG),
        );
    });

    ui.add_space(20.0);

    let popular: Vec<_> = POPULAR_STATION_IDS
        .iter()
        .filter_map(|id| stations.iter().find(|s| s.id == *id))
        .collect();

    let shadow = Shadow {
        offset: [0, 2],
        blur: 8,
        spread: 0,
        color: colors::SHADOW,
    };

    ui.columns_const(|cols: &mut [Ui; 5]| {
        for (col, station) in cols.iter_mut().zip(&popular) {
            let title = if is_turkmen {
                &station.title_tm
            } else {
                &station.title_ru
            };

            let is_selected = selected_id == Some(station.id);
            let is_disabled = other_id == Some(station.id);

            let (bg, fg, show_shadow) = if is_selected {
                (colors::BTN_PRIMARY_BG, colors::WHITE, false)
            } else if is_disabled {
                (colors::BG_5, colors::FG_DISABLED, false)
            } else {
                (colors::WHITE, colors::FG, true)
            };

            let width = col.available_width() - 10.0;
            let sense = if is_disabled {
                Sense::empty()
            } else {
                Sense::CLICK
            };

            col.vertical_centered(|ui| {
                let (rect, res) = ui.allocate_exact_size(vec2(width, 80.0), sense);

                if show_shadow {
                    ui.painter().add(shadow.as_shape(rect, corners::MEDIUM));
                }

                ui.painter().rect_filled(rect, corners::MEDIUM, bg);
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    title,
                    FontId::proportional(18.0),
                    fg,
                );

                if res.clicked() && !is_disabled {
                    state.trips.selected = true;

                    let station = (*station).clone();

                    if is_source_modal {
                        state.trips.source = Some(station);
                    } else {
                        state.trips.destination = Some(station);
                    }
                }
            });
        }
    });
}

fn station_id_for(state: &State, source: bool) -> Option<u32> {
    if source {
        state.trips.source.as_ref().map(|s| s.id)
    } else {
        state.trips.destination.as_ref().map(|s| s.id)
    }
}

fn render_letters(state: &mut State, ui: &mut Ui) {
    let rows = if state.lang.is_turkmen() {
        alphabet::TM
    } else {
        alphabet::RU
    };

    const PADDING: f32 = 5.0;
    const BTN_SIZE: f32 = 78.0;

    let shadow = Shadow {
        offset: [0, 2],
        blur: 6,
        spread: 0,
        color: colors::SHADOW,
    };

    for row in rows {
        ui.horizontal(|ui| {
            for letter in row.iter().filter(|l| !l.is_empty()) {
                let is_selected = state.stations.selected_letter == *letter;

                let (bg_color, fg_color) = if is_selected {
                    (colors::BTN_PRIMARY_BG, colors::WHITE)
                } else {
                    (colors::WHITE, colors::FG)
                };

                let (rect, res) = ui.allocate_exact_size(vec2(BTN_SIZE, BTN_SIZE), Sense::CLICK);

                if !is_selected {
                    ui.painter().add(shadow.as_shape(rect, corners::MEDIUM));
                }

                ui.painter().rect_filled(rect, corners::MEDIUM, bg_color);
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    *letter,
                    FontId::proportional(20.0),
                    fg_color,
                );

                if res.clicked() {
                    state.stations.selected_letter = letter;
                }

                ui.add_space(PADDING);
            }
        });

        ui.add_space(PADDING);
    }
}

fn render_stations(state: &mut State, ui: &mut Ui) {
    let frame = Frame::new()
        .inner_margin(10)
        .corner_radius(corners::MEDIUM)
        .fill(colors::WHITE)
        .shadow(Shadow {
            offset: [0, 2],
            blur: 8,
            spread: 0,
            color: colors::SHADOW,
        });

    let is_source_modal = state.modal == ModalKind::Source;
    let selected_id = station_id_for(state, is_source_modal);
    let other_id = station_id_for(state, !is_source_modal);

    let shadow = Shadow {
        offset: [0, 2],
        blur: 6,
        spread: 0,
        color: colors::SHADOW,
    };

    frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.set_height(ui.available_height());

        ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                let Some(stations) = state.stations.get() else {
                    return;
                };

                let is_turkmen = state.lang.is_turkmen();
                let selected_letter = state.stations.selected_letter;

                for station in stations {
                    let (title, starts_with) = if is_turkmen {
                        (
                            &station.title_tm,
                            station.title_tm.starts_with(selected_letter),
                        )
                    } else {
                        (
                            &station.title_ru,
                            station.title_ru.starts_with(selected_letter),
                        )
                    };

                    if !starts_with {
                        continue;
                    }

                    let is_selected = selected_id == Some(station.id);
                    let is_disabled = other_id == Some(station.id);

                    let (bg, fg, show_shadow) = if is_selected {
                        (colors::BTN_PRIMARY_BG, colors::WHITE, false)
                    } else if is_disabled {
                        (colors::BG_5, colors::FG_DISABLED, false)
                    } else {
                        (colors::WHITE, colors::FG, true)
                    };

                    let sense = if is_disabled {
                        Sense::empty()
                    } else {
                        Sense::CLICK
                    };

                    let (rect, res) =
                        ui.allocate_exact_size(vec2(ui.available_width(), 80.0), sense);

                    if show_shadow {
                        ui.painter().add(shadow.as_shape(rect, corners::MEDIUM));
                    }

                    ui.painter().rect_filled(rect, corners::MEDIUM, bg);
                    ui.painter().text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        title,
                        FontId::proportional(18.0),
                        fg,
                    );

                    if res.clicked() && !is_disabled {
                        let station = station.clone();

                        if is_source_modal {
                            state.trips.source = Some(station);
                        } else {
                            state.trips.destination = Some(station);
                        }

                        state.trips.selected = true;
                    }

                    ui.add_space(10.0);
                }
            });
        });
    });
}
