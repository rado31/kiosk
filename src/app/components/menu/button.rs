use super::*;

pub struct Button<'a> {
    pub icon_path: ImageSource<'a>,
    pub route: Route,
    pub color: Color32,
}

impl<'a> Button<'a> {
    pub fn new(icon_path: ImageSource<'a>, route: Route, color: Color32) -> Self {
        Self {
            icon_path,
            route,
            color,
        }
    }

    pub fn label(&self) -> &'static str {
        match self.route {
            Route::Home => "Home",
            Route::PrintTicket => "Print",
            Route::Refund => "Refund",
            Route::Seats => "Seats",
        }
    }

    pub fn show(&self, ui: &mut Ui, state: &mut State) {
        let btn_size = vec2(150.0, 100.0);
        let img_size = vec2(25.0, 25.0);
        let corner_radius = CornerRadius::from(12);

        let is_active = state.current_route == self.route;

        let (bg_active, fg_active, tint) = if is_active {
            (constants::PRIMARY, constants::WHITE, constants::WHITE)
        } else {
            (constants::WHITE, constants::BLACK, self.color)
        };

        let (rect, _response) = ui.allocate_exact_size(btn_size, Sense::click());

        ui.painter().rect(
            rect,
            corner_radius,
            bg_active,
            Stroke::new(1.0, constants::BORDER),
            StrokeKind::Outside,
        );

        let mut child_ui = ui.new_child(UiBuilder::new().max_rect(rect.shrink(20.0)));

        child_ui.vertical_centered(|ui| {
            let img = Image::new(self.icon_path.clone())
                .fit_to_exact_size(img_size)
                .tint(tint);

            ui.add(img);
            ui.add_space(10.0);
            ui.label(
                RichText::new(self.label())
                    .color(fg_active)
                    .strong()
                    .size(16.0),
            );
        });

        if utils::rect_is_clicked(ui, rect) {
            state.current_route = self.route;
        }
    }
}
