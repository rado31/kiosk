use egui::{
    Align, FontFamily, FontId, Frame, Id, Margin, Response, Stroke, TextEdit, Ui, Vec2, vec2,
};

use crate::theme::{colors, corners};

pub struct Input<'a> {
    value: &'a mut String,
    hint: String,
    font: FontId,
    char_limit: Option<usize>,
    horizontal_align: Align,
    vertical_align: Align,
    desired_size: Option<Vec2>,
    id: Option<Id>,
}

#[allow(dead_code)]
impl<'a> Input<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            value,
            hint: String::new(),
            font: FontId::new(16.0, FontFamily::Proportional),
            char_limit: None,
            horizontal_align: Align::LEFT,
            vertical_align: Align::Center,
            desired_size: None,
            id: None,
        }
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = hint.into();
        self
    }

    pub fn font(mut self, font: FontId) -> Self {
        self.font = font;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font.size = size;
        self
    }

    pub fn bold(mut self) -> Self {
        self.font.family = FontFamily::Name("bold".into());
        self
    }

    pub fn char_limit(mut self, limit: usize) -> Self {
        self.char_limit = Some(limit);
        self
    }

    pub fn horizontal_align(mut self, align: Align) -> Self {
        self.horizontal_align = align;
        self
    }

    pub fn vertical_align(mut self, align: Align) -> Self {
        self.vertical_align = align;
        self
    }

    pub fn desired_size(mut self, size: Vec2) -> Self {
        self.desired_size = Some(size);
        self
    }

    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let id = self.id.unwrap_or_else(|| ui.next_auto_id());
        let has_focus = ui.memory(|m| m.has_focus(id));

        let border_color = if has_focus {
            colors::PRIMARY
        } else {
            colors::BORDER
        };

        let frame_response = Frame::new()
            .inner_margin(Margin::symmetric(12, 10))
            .corner_radius(corners::MEDIUM)
            .fill(colors::INPUT_BG)
            .stroke(Stroke::new(1.0, border_color))
            .show(ui, |ui| {
                let size = self.desired_size.unwrap_or_else(|| {
                    let w = match self.char_limit {
                        Some(limit) => self.font.size * limit as f32,
                        None => ui.available_width(),
                    };

                    vec2(w, self.font.size * 1.8)
                });

                ui.set_width(size.x);
                ui.set_height(size.y);

                let mut input = TextEdit::singleline(self.value)
                    .id(id)
                    .hint_text(self.hint)
                    .font(self.font)
                    .text_color(colors::FG)
                    .background_color(colors::INPUT_BG)
                    .horizontal_align(self.horizontal_align)
                    .vertical_align(self.vertical_align)
                    .min_size(size)
                    .frame(false);

                if let Some(limit) = self.char_limit {
                    input = input.char_limit(limit);
                }

                ui.add(input)
            });

        frame_response.inner
    }
}
