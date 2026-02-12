use chrono::{Datelike, NaiveDate};
use egui::{Align2, Context, FontFamily, FontId, Sense, Shadow, Ui, vec2};

use crate::{
    components::modal::Modal,
    i18n::{Language, t},
    state::calendar,
    theme::{colors, corners},
};

#[derive(Clone, Copy, PartialEq)]
pub enum CalendarKind {
    OneWay,
    RoundTrip,
}

pub struct Calendar<'a> {
    id: &'a str,
    state: &'a mut calendar::State,
    lang: &'a Language,
    ctx: &'a Context,
    kind: CalendarKind,
}

impl<'a> Calendar<'a> {
    pub fn new(
        id: &'a str,
        state: &'a mut calendar::State,
        lang: &'a Language,
        ctx: &'a Context,
        kind: CalendarKind,
    ) -> Self {
        Self {
            id,
            state,
            lang,
            ctx,
            kind,
        }
    }

    /// Shows the calendar modal. Returns `true` if modal should close.
    pub fn show(&mut self) -> bool {
        let today = calendar::today();
        let mut date_selected = false;

        let should_close = Modal::new(self.id).width(700.0).open(self.ctx, |ui| {
            self.render_header(ui, today);
            ui.add_space(16.0);
            self.render_day_headers(ui);
            ui.add_space(8.0);
            date_selected = self.render_day_grid(ui, today);
        });

        should_close || date_selected
    }

    fn selected_date(&self) -> NaiveDate {
        match self.kind {
            CalendarKind::OneWay => self.state.one_way_date,
            CalendarKind::RoundTrip => self.state.round_trip_date,
        }
    }

    fn set_selected_date(&mut self, date: NaiveDate) {
        match self.kind {
            CalendarKind::OneWay => self.state.set_one_way_date(date),
            CalendarKind::RoundTrip => self.state.set_round_trip_date(date),
        }
    }

    fn min_selectable_date(&self, today: NaiveDate) -> NaiveDate {
        match self.kind {
            CalendarKind::OneWay => today,
            CalendarKind::RoundTrip => self.state.one_way_date.succ_opt().unwrap(),
        }
    }

    fn render_header(&mut self, ui: &mut Ui, today: NaiveDate) {
        let earliest = match self.kind {
            CalendarKind::OneWay => today,
            CalendarKind::RoundTrip => self.state.one_way_date,
        };

        let can_go_prev = self.state.viewed_year > earliest.year()
            || (self.state.viewed_year == earliest.year()
                && self.state.viewed_month > earliest.month());

        let width = ui.available_width();
        let btn_size = 56.0;
        let corner = btn_size / 2.0;

        ui.horizontal(|ui| {
            let (prev_rect, prev_res) =
                ui.allocate_exact_size(vec2(btn_size, btn_size), Sense::CLICK);

            let (prev_bg, prev_fg) = if can_go_prev {
                (colors::BG_DIM, colors::PRIMARY)
            } else {
                (colors::BG_DIM, colors::FG_DISABLED)
            };

            ui.painter().rect_filled(prev_rect, corner, prev_bg);
            ui.painter().text(
                prev_rect.center() - vec2(1.0, 0.0),
                Align2::CENTER_CENTER,
                "\u{2039}",
                FontId::new(28.0, FontFamily::Name("bold".into())),
                prev_fg,
            );

            if can_go_prev && prev_res.clicked() {
                self.state.prev_month();
            }

            let month_key = format!("month_{}", self.state.viewed_month);
            let month_name = t(self.lang, &month_key);
            let label = format!("{} {}", month_name, self.state.viewed_year);
            let label_width = width - btn_size * 2.0;
            let (label_rect, _) =
                ui.allocate_exact_size(vec2(label_width, btn_size), Sense::empty());

            ui.painter().text(
                label_rect.center(),
                Align2::CENTER_CENTER,
                label,
                FontId::new(28.0, FontFamily::Name("bold".into())),
                colors::FG,
            );

            let (next_rect, next_res) =
                ui.allocate_exact_size(vec2(btn_size, btn_size), Sense::CLICK);

            ui.painter().rect_filled(next_rect, corner, colors::BG_DIM);
            ui.painter().text(
                next_rect.center() + vec2(1.0, 0.0),
                Align2::CENTER_CENTER,
                "\u{203A}",
                FontId::new(28.0, FontFamily::Name("bold".into())),
                colors::PRIMARY,
            );

            if next_res.clicked() {
                self.state.next_month();
            }
        });
    }

    fn render_day_headers(&self, ui: &mut Ui) {
        let day_keys = [
            "day_mon", "day_tue", "day_wed", "day_thu", "day_fri", "day_sat", "day_sun",
        ];

        let cell_size = ui.available_width() / 7.0;

        ui.horizontal(|ui| {
            for key in &day_keys {
                let (rect, _) = ui.allocate_exact_size(vec2(cell_size, 40.0), Sense::empty());

                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    t(self.lang, key),
                    FontId::new(18.0, FontFamily::Name("bold".into())),
                    colors::FG_MUTED,
                );
            }
        });
    }

    fn render_day_grid(&mut self, ui: &mut Ui, today: NaiveDate) -> bool {
        let selected = self.selected_date();
        let min_date = self.min_selectable_date(today);
        let year = self.state.viewed_year;
        let month = self.state.viewed_month;
        let days_in_current = calendar::days_in_month(year, month);
        let first_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let first_weekday = calendar::weekday_index(first_date);

        let (prev_year, prev_month) = if month == 1 {
            (year - 1, 12)
        } else {
            (year, month - 1)
        };

        let days_in_prev = calendar::days_in_month(prev_year, prev_month);
        let cell_size = ui.available_width() / 7.0;
        let cell_height = 70.0;
        let circle_radius = 28.0;
        let mut date_selected = false;

        let shadow = Shadow {
            offset: [0, 1],
            blur: 4,
            spread: 0,
            color: colors::SHADOW,
        };

        for row in 0..6 {
            ui.horizontal(|ui| {
                for col in 0..7 {
                    let idx = row * 7 + col;
                    let (rect, response) =
                        ui.allocate_exact_size(vec2(cell_size, cell_height), Sense::CLICK);

                    let center = rect.center();

                    if idx < first_weekday {
                        let d = days_in_prev - (first_weekday - 1 - idx);

                        ui.painter().text(
                            center,
                            Align2::CENTER_CENTER,
                            format!("{d}"),
                            FontId::proportional(24.0),
                            colors::FG_DISABLED,
                        );

                        continue;
                    }

                    let day = idx - first_weekday + 1;

                    if day > days_in_current {
                        let d = day - days_in_current;
                        ui.painter().text(
                            center,
                            Align2::CENTER_CENTER,
                            format!("{d}"),
                            FontId::proportional(24.0),
                            colors::FG_DISABLED,
                        );

                        continue;
                    }

                    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                    let is_selected = date == selected;
                    let is_today = date == today;
                    let is_past = date < min_date;

                    if is_selected {
                        ui.painter().add(shadow.as_shape(
                            rect.shrink2(vec2(
                                cell_size / 2.0 - circle_radius,
                                cell_height / 2.0 - circle_radius,
                            )),
                            corners::LARGE,
                        ));

                        ui.painter()
                            .circle_filled(center, circle_radius, colors::PRIMARY);

                        ui.painter().text(
                            center,
                            Align2::CENTER_CENTER,
                            format!("{day}"),
                            FontId::new(24.0, FontFamily::Name("bold".into())),
                            colors::WHITE,
                        );
                    } else if is_today {
                        ui.painter()
                            .circle_filled(center, circle_radius, colors::PRIMARY_BG);

                        ui.painter().text(
                            center,
                            Align2::CENTER_CENTER,
                            format!("{day}"),
                            FontId::new(24.0, FontFamily::Name("bold".into())),
                            colors::PRIMARY,
                        );
                    } else {
                        let text_color = if is_past {
                            colors::FG_DISABLED
                        } else {
                            colors::FG
                        };

                        ui.painter().text(
                            center,
                            Align2::CENTER_CENTER,
                            format!("{day}"),
                            FontId::proportional(24.0),
                            text_color,
                        );
                    }

                    if !is_past && response.clicked() {
                        self.set_selected_date(date);
                        date_selected = true;
                    }
                }
            });
        }

        date_selected
    }
}
