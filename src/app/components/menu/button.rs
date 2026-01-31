use super::*;

pub struct Button<'a> {
    pub icon_path: ImageSource<'a>,
    pub view: View,
    pub color: Color32,
}

impl<'a> Button<'a> {
    pub fn new(icon_path: ImageSource<'a>, view: View, color: Color32) -> Self {
        Self {
            icon_path,
            view,
            color,
        }
    }

    fn label(&self, lang: Language) -> &'static str {
        match self.view {
            View::Home => t(lang, "home"),
            View::PrintTicket => t(lang, "print_ticket"),
            View::Refund => t(lang, "refund"),
            View::Seats => t(lang, "seats"),
            View::History => t(lang, "history"),
        }
    }

    pub fn show(&self, ui: &mut Ui, state: &mut State) {
        let btn_size = vec2(150.0, 100.0);
        let img_size = vec2(25.0, 25.0);
        let is_active = state.view == self.view;

        let (bg_active, fg_active, tint) = if is_active {
            (colors::PRIMARY, colors::WHITE, colors::WHITE)
        } else {
            (colors::WHITE, colors::BLACK, self.color)
        };

        let (rect, res) = ui.allocate_exact_size(btn_size, Sense::CLICK);

        ui.painter().rect(
            rect,
            corners::LARGE,
            bg_active,
            Stroke::new(1.0, colors::BORDER),
            StrokeKind::Outside,
        );

        let mut child_ui = ui.new_child(UiBuilder::new().max_rect(rect));

        child_ui.vertical_centered(|ui| {
            ui.add_space(20.0);

            let img = Image::new(self.icon_path.clone())
                .fit_to_exact_size(img_size)
                .tint(tint);

            ui.add(img);
            ui.add_space(10.0);
            ui.label(
                RichText::new(self.label(state.lang))
                    .color(fg_active)
                    .size(16.0),
            );
        });

        if res.clicked() {
            state.view = self.view;
        }
    }
}
