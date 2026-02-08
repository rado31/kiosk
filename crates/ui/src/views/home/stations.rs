use egui::{Button, Context, Frame, RichText, ScrollArea, Stroke, Ui, vec2};

use crate::{
    components::modal::Modal,
    state::{State, modal::Modal as ModalKind},
    theme::{alphabet, colors, corners},
};

pub fn show(state: &mut State, ctx: &Context) {
    if state.modal == ModalKind::Source || state.modal == ModalKind::Destination {
        let should_close = Modal::new("stations_modal")
            .width(880.0)
            .open(ctx, |ui| {
                ui.columns_const(|[col1, col2]| {
                    col1.vertical(|ui| render_letters(state, ui));
                    col2.vertical(|ui| render_stations(state, ui));
                });
            });

        if should_close || state.trip.selected {
            state.modal = ModalKind::Closed;
            state.trip.selected = false;
        };
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

    for row in rows {
        ui.horizontal(|ui| {
            for letter in row.iter().filter(|l| !l.is_empty()) {
                let is_selected = state.stations.selected_letter == *letter;

                let (bg_color, fg_color) = if is_selected {
                    (colors::BTN_PRIMARY_BG, colors::WHITE)
                } else {
                    (colors::WHITE, colors::BLACK)
                };

                let btn = Button::new(RichText::new(*letter).size(20.0).color(fg_color))
                    .min_size(vec2(BTN_SIZE, BTN_SIZE))
                    .stroke(Stroke::new(1.0, colors::BORDER))
                    .fill(bg_color)
                    .corner_radius(corners::MEDIUM);

                if ui.add(btn).clicked() {
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
        .corner_radius(8)
        .stroke(Stroke::new(1.0, colors::BORDER));

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

                    let btn = Button::new(RichText::new(title).size(18.0).color(colors::BLACK))
                        .min_size(vec2(250.0, 80.0))
                        .fill(colors::BG_4)
                        .corner_radius(corners::MEDIUM);

                    if ui.add(btn).clicked() {
                        let station = station.clone();

                        if state.modal == ModalKind::Source {
                            state.trip.source = Some(station);
                        } else {
                            state.trip.destination = Some(station);
                        }

                        state.trip.selected = true;
                    }

                    ui.add_space(10.0);
                }
            });
        });
    });
}
