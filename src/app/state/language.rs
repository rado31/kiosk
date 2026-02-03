#[derive(Default, Clone, Copy, PartialEq)]
pub enum Kind {
    #[default]
    Turkmen,
    Russian,
}

#[derive(Default)]
pub struct State {
    lang: Kind,
}

impl State {
    pub fn is_turkmen(&self) -> bool {
        matches!(self.lang, Kind::Turkmen)
    }

    pub fn get(&self) -> &Kind {
        &self.lang
    }

    pub fn toggle(&mut self) {
        self.lang = if self.is_turkmen() {
            Kind::Russian
        } else {
            Kind::Turkmen
        };
    }
}
