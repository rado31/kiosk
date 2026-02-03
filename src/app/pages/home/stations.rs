use egui::{Button, Frame, RichText, ScrollArea, Stroke, Ui, vec2};

use crate::app::{
    components::base::Modal,
    constants::{alphabet, colors, corners},
};

use super::Home;

impl<'a> Home<'a> {
    pub fn show_stations(&mut self) {
        if self.state.modal.is_source() || self.state.modal.is_destination() {
            let should_close = Modal::new("pnr_counts_modal")
                .width(880.0)
                .open(self.ctx, |ui| {
                    ui.columns_const(|[col1, col2]| {
                        col1.vertical(|ui| self.render_letters(ui));
                        col2.vertical(|ui| self.render_stations(ui));
                    });
                });

            if should_close || self.state.trip.is_selected() {
                self.state.modal.close();
                self.state.trip.selected(false);
            };
        }
    }

    fn render_letters(&mut self, ui: &mut Ui) {
        let rows = if self.state.lang.is_turkmen() {
            alphabet::TM
        } else {
            alphabet::RU
        };

        const PADDING: f32 = 5.0;
        const BTN_SIZE: f32 = 78.0;

        for row in rows {
            ui.horizontal(|ui| {
                for letter in row.iter().filter(|l| !l.is_empty()) {
                    let is_selected = self.state.stations.get_letter() == *letter;

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
                        self.state.stations.select_letter(letter);
                    }

                    ui.add_space(PADDING);
                }
            });

            ui.add_space(PADDING);
        }
    }

    pub fn render_stations(&mut self, ui: &mut Ui) {
        let frame = Frame::new()
            .inner_margin(10)
            .corner_radius(8)
            .stroke(Stroke::new(1.0, colors::BORDER));

        frame.show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_height(ui.available_height());

            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    let Some(stations) = self.state.stations.get() else {
                        return;
                    };

                    let is_turkmen = self.state.lang.is_turkmen();
                    let selected_letter = self.state.stations.get_letter();

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

                            if self.state.modal.is_source() {
                                self.state.trip.set_source(station);
                            } else {
                                self.state.trip.set_destination(station);
                            }

                            self.state.trip.selected(true);
                        }

                        ui.add_space(10.0);
                    }
                });
            });
        });
    }
}
