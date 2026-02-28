use egui::{Align, Align2, FontFamily, FontId, Frame, Layout, RichText, Sense, Shadow, Ui, vec2};

use crate::{
    i18n::t,
    state::State,
    theme::{colors, corners},
};

pub(super) fn render(state: &mut State, ui: &mut Ui) {
    let title = RichText::new(t(&state.lang, "selected_passenger_seats"))
        .size(18.0)
        .family(FontFamily::Name("bold".into()))
        .color(colors::FG);

    ui.label(title);
    ui.add_space(12.0);

    ui.columns_const(|[col1, col2]| {
        col1.vertical(|ui| {
            let total = state.passengers.total() as usize;

            for i in 0..total {
                let seat_label = state
                    .seats
                    .selected
                    .get(i)
                    .and_then(|s| s.as_ref())
                    .map(|s| s.seat_label.clone());

                let card = Frame::new()
                    .inner_margin(14.0)
                    .fill(colors::WHITE)
                    .corner_radius(corners::MEDIUM)
                    .shadow(Shadow {
                        offset: [0, 2],
                        blur: 8,
                        spread: 0,
                        color: colors::SHADOW,
                    });

                card.show(ui, |ui| {
                    ui.set_width(230.0);

                    ui.horizontal(|ui| {
                        let (icon_rect, _) =
                            ui.allocate_exact_size(vec2(24.0, 24.0), Sense::empty());

                        ui.painter()
                            .circle_filled(icon_rect.center(), 12.0, colors::PRIMARY_BG);

                        ui.painter().text(
                            icon_rect.center(),
                            Align2::CENTER_CENTER,
                            "✦",
                            FontId::proportional(10.0),
                            colors::PRIMARY,
                        );

                        ui.add_space(8.0);

                        let name = format!("{} {}", t(&state.lang, "passenger"), i + 1);
                        ui.label(RichText::new(name).size(16.0).color(colors::FG));

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let display = seat_label.as_deref().unwrap_or("-");
                            let text = format!("{}: {}", t(&state.lang, "seat_label"), display);
                            let lbl = RichText::new(text)
                                .size(16.0)
                                .family(FontFamily::Name("bold".into()))
                                .color(colors::FG);

                            ui.label(lbl);
                        });
                    });
                });

                ui.add_space(12.0);
            }
        });

        col2.with_layout(Layout::right_to_left(Align::Min), |ui| {
            let can_proceed = state.seats.all_assigned();
            let (btn_bg, btn_fg) = if can_proceed {
                (colors::BTN_PRIMARY_BG, colors::WHITE)
            } else {
                (colors::BG_5, colors::FG_DISABLED)
            };

            let sense = if can_proceed {
                Sense::CLICK
            } else {
                Sense::empty()
            };

            let btn_text = format!("{} →", t(&state.lang, "next_page"));
            let (btn_rect, btn_res) = ui.allocate_exact_size(vec2(220.0, 54.0), sense);

            ui.painter().rect_filled(btn_rect, corners::MEDIUM, btn_bg);
            ui.painter().text(
                btn_rect.center(),
                Align2::CENTER_CENTER,
                &btn_text,
                FontId::new(18.0, FontFamily::Name("bold".into())),
                btn_fg,
            );

            if btn_res.clicked() {
                log::debug!("proceed to next page");
            }
        });
    });
}
