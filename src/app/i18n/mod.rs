use super::state::language::Kind;

mod russian;
mod turkmen;

pub fn t(lang: &Kind, key: &str) -> &'static str {
    let map = match lang {
        Kind::Turkmen => &turkmen::TRANSLATIONS,
        Kind::Russian => &russian::TRANSLATIONS,
    };

    map.get(key).copied().unwrap_or("???")
}
