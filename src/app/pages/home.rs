use super::*;

pub fn show(state: &mut State, ui: &mut Ui) {
    Frame::new()
        .inner_margin(10)
        .corner_radius(12)
        .fill(colors::WHITE)
        .stroke(Stroke::new(1.0, colors::BORDER))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            Frame::new()
                .inner_margin(5)
                .corner_radius(8)
                .stroke(Stroke::new(1.0, colors::BORDER))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Allocate space for both tabs first
                        let (rect_1, response_1) =
                            ui.allocate_exact_size(egui::vec2(150.0, 40.0), Sense::click());

                        ui.add_space(10.0);

                        let (rect_2, response_2) =
                            ui.allocate_exact_size(egui::vec2(150.0, 40.0), Sense::click());

                        if response_1.clicked() {
                            state.set_one_way();
                        }

                        if response_2.clicked() {
                            state.set_round_trip();
                        }

                        // 1. Draw background rectangle FIRST
                        let target_rect = if state.is_one_way() { rect_1 } else { rect_2 };
                        let animation_id = ui.id().with("tab_indicator");
                        let current_x = ui.ctx().animate_value_with_time(
                            animation_id.with("x"),
                            target_rect.min.x,
                            0.2,
                        );

                        let animated_rect = Rect::from_min_size(
                            egui::pos2(current_x, target_rect.min.y),
                            egui::vec2(target_rect.width(), target_rect.height()),
                        );

                        ui.painter().rect_filled(
                            animated_rect,
                            CornerRadius::same(8),
                            colors::BTN_PRIMARY_BG,
                        );

                        let (text_color_1, text_color_2) = if state.is_one_way() {
                            (colors::WHITE, colors::BLACK)
                        } else {
                            (colors::BLACK, colors::WHITE)
                        };

                        // 2. Draw text labels on TOP
                        ui.painter().text(
                            rect_1.center(),
                            egui::Align2::CENTER_CENTER,
                            t(state.language, "one_way"),
                            egui::FontId::proportional(14.0),
                            text_color_1,
                        );

                        ui.painter().text(
                            rect_2.center(),
                            egui::Align2::CENTER_CENTER,
                            t(state.language, "round_trip"),
                            egui::FontId::proportional(14.0),
                            text_color_2,
                        );
                    });
                });
        });
}
