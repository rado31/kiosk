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

    // Trip
    "one_way" => "В одну сторону",
    "round_trip" => "Туда и Обратно",
    "search" => "Поиск",

    // Passenger
    "pnrs_count" => "Количество пассажиров",
    "pnrs_max" => "Максимальное количество пассажиров 9",
    "adult" => "Взрослый (10+ лет)",
    "child" => "Ребёнок (5-10 лет)",

    // Days of week (2-letter)
    "day_mon" => "Пн",
    "day_tue" => "Вт",
    "day_wed" => "Ср",
    "day_thu" => "Чт",
    "day_fri" => "Пт",
    "day_sat" => "Сб",
    "day_sun" => "Вс",

    // Month names
    "month_1" => "Январь",
    "month_2" => "Февраль",
    "month_3" => "Март",
    "month_4" => "Апрель",
    "month_5" => "Май",
    "month_6" => "Июнь",
    "month_7" => "Июль",
    "month_8" => "Август",
    "month_9" => "Сентябрь",
    "month_10" => "Октябрь",
    "month_11" => "Ноябрь",
    "month_12" => "Декабрь",

    // Stations
    "most_popular_places" => "Самые популярные места",
    "find_by_letters" => "Вы также можете найти по буквам",
    "from" => "Откуда",
    "to" => "Куда",
    "stations_fetch_error" => "Не удалось загрузить станции",

    // Notifications
    "select_source" => "Выберите станцию отправления",
    "select_destination" => "Выберите станцию назначения",

    // Trips
    "trips_not_found" => "Рейсы не найдены",
    "trips_fetch_error" => "Не удалось загрузить рейсы",
    "departure" => "Отправление",
    "arrival" => "Прибытие",
    "price" => "Цена",
    "back" => "Назад",
    "km" => "км",
    "hour_short" => "ч",
    "min_short" => "мин",

    // Print ticket
    "enter_booking_number" => "Введите номер бронирования",
    "print" => "Распечатать",
    "terminal_ticket" => "Терминал",
    "external_ticket" => "Касса / Онлайн",
    "external_print_notice" => "Печать этих билетов платная",
    "pay_and_print" => "Оплатить и распечатать",

    // Seats
    "outbound_leg" => "Туда",
    "inbound_leg" => "Обратно",
    "choose_seat" => "Выберите место пассажира",
    "choose_seat_hint" => "Выберите место согласно схеме, показанной ниже",
    "explanation" => "Объяснение",
    "free_seats" => "Свободные места",
    "taken_seats" => "Занятые места",
    "your_seats" => "Ваши выбранные",
    "selected_passenger_seats" => "Выбранные места пассажиров",
    "passenger" => "Пассажир",
    "seat_label" => "Место",
    "wagon" => "Вагон",
    "next_page" => "Следующая страница",
    "floor_1" => "1-й этаж",
    "floor_2" => "2-й этаж",
    "floor_3" => "3-й этаж",
    "seats_fetch_error" => "Не удалось загрузить места",
};
