mod russian;
mod turkmen;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Language {
    #[default]
    Turkmen,
    Russian,
}

impl Language {
    pub fn is_turkmen(&self) -> bool {
        matches!(self, Language::Turkmen)
    }

    pub fn toggle(&mut self) {
        *self = if self.is_turkmen() {
            Language::Russian
        } else {
            Language::Turkmen
        };
    }
}

pub fn t(lang: &Language, key: &str) -> &'static str {
    let map = match lang {
        Language::Turkmen => &turkmen::TRANSLATIONS,
        Language::Russian => &russian::TRANSLATIONS,
    };

    map.get(key).copied().unwrap_or("???")
}
