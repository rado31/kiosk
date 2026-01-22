use super::*;

pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    // Menu buttons
    "home" => "Baş sahypa",
    "print_ticket" => "Bilet çap et",
    "refund" => "Yzyna gaýtarmak",
    "seats" => "Ýerler",

    // Terminal title
    "terminal_title" => "Otly biletlerini satyn almak üçin terminal",

    // Page titles
    "home_page" => "Baş sahypa",
    "print_ticket_page" => "Bilet çap etmek",
    "refund_page" => "Yzyna gaýtarmak",
    "seats_page" => "Ýerleri saýlamak",
};
