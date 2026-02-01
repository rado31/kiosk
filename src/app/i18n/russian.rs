use phf::phf_map;

pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    // Menu buttons
    "home" => "Главная",
    "print_ticket" => "Печать билета",
    "refund" => "Возврат",
    "seats" => "Места",
    "history" => "История",

    // Terminal title
    "terminal_title" => "Терминал для покупки ж/д билетов",

    // Page titles
    "home_page" => "Главная страница",
    "print_ticket_page" => "Печать билета",
    "refund_page" => "Возврат билета",
    "seats_page" => "Выбор мест",

    // Type of trip
    "one_way" => "В одну сторону",
    "round_trip" => "Туда и Обратно",

    // Passenger
    "pnr" => "Пассажир",
    "adult" => "Взрослый",
    "child" => "Ребёнок",
};
