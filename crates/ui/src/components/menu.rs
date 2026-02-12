use egui::{
    Color32, FontFamily, Image, ImageSource, RichText, Sense, Shadow, Stroke, StrokeKind, Ui,
    UiBuilder, include_image, vec2,
};

use crate::{
    i18n::{Language, t},
    state::State,
    theme::{colors, corners},
    views::View,
};

pub fn show(state: &mut State, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        let title = RichText::new(t(&state.lang, "terminal_title"))
            .size(32.0)
            .family(FontFamily::Name("bold".into()))
            .color(colors::FG);

        ui.label(title)
    });

    let buttons = [
        MenuButton::new(
            include_image!("../assets/ticket-check.svg"),
            View::Home,
            colors::PRIMARY,
        ),
        MenuButton::new(
            include_image!("../assets/printer-check.svg"),
            View::PrintTicket,
            colors::BTN_GREEN,
        ),
        MenuButton::new(
            include_image!("../assets/ticket-x.svg"),
            View::Refund,
            colors::BTN_RED,
        ),
        MenuButton::new(
            include_image!("../assets/ticket-check.svg"),
            View::History,
            colors::SECONDARY,
        ),
    ];

    ui.add_space(30.0);

    ui.horizontal(|ui| {
        ui.style_mut().spacing.item_spacing.x = 10.0;

        for button in buttons.iter() {
            button.render(ui, state);
        }
    });

    ui.add_space(20.0);
}

struct MenuButton<'a> {
    icon_path: ImageSource<'a>,
    view: View,
    color: Color32,
}

impl<'a> MenuButton<'a> {
    fn new(icon_path: ImageSource<'a>, view: View, color: Color32) -> Self {
        Self {
            icon_path,
            view,
            color,
        }
    }

    fn label(&self, lang: &Language) -> &'static str {
        match self.view {
            View::Home => t(lang, "home"),
            View::PrintTicket => t(lang, "print_ticket"),
            View::Refund => t(lang, "refund"),
            View::Seats => t(lang, "seats"),
            View::History => t(lang, "history"),
            _ => "",
        }
    }

    fn render(&self, ui: &mut Ui, state: &mut State) {
        let btn_size = vec2(150.0, 100.0);
        let img_size = vec2(25.0, 25.0);
        let is_active = state.current_view() == self.view;

        let (bg_active, fg_active, tint) = if is_active {
            (colors::PRIMARY, colors::WHITE, colors::WHITE)
        } else {
            (colors::WHITE, colors::FG, self.color)
        };

        let (rect, res) = ui.allocate_exact_size(btn_size, Sense::CLICK);

        let shadow = Shadow {
            offset: [0, 2],
            blur: 8,
            spread: 0,
            color: colors::SHADOW,
        };

        ui.painter().add(shadow.as_shape(rect, corners::LARGE));
        ui.painter().rect(
            rect,
            corners::LARGE,
            bg_active,
            Stroke::NONE,
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

            let lbl = RichText::new(self.label(&state.lang))
                .color(fg_active)
                .size(16.0);

            ui.label(lbl);
        });

        if res.clicked() {
            state.go_to(self.view);
        }
    }
}
