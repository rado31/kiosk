mod russian;
mod turkmen;

use super::Language;

pub fn t(lang: Language, key: &str) -> &'static str {
    let map = match lang {
        Language::Turkmen => &turkmen::TRANSLATIONS,
        Language::Russian => &russian::TRANSLATIONS,
    };

    map.get(key).copied().unwrap_or("???")
}
