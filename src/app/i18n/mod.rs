use phf::phf_map;

use super::state::Language;

mod russian;
mod turkmen;

pub fn t(lang: Language, key: &str) -> &'static str {
    let map = match lang {
        Language::Turkmen => &turkmen::TRANSLATIONS,
        Language::Russian => &russian::TRANSLATIONS,
    };

    map.get(key).copied().unwrap_or("???")
}
