use phf::phf_map;

pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    // Menu buttons
    "home" => "Baş sahypa",
    "print_ticket" => "Bilet çap et",
    "refund" => "Yzyna gaýtarmak",
    "seats" => "Ýerler",
    "history" => "Taryh",

    // Terminal title
    "terminal_title" => "Otly biletlerini satyn almak üçin terminal",

    // Page titles
    "home_page" => "Baş sahypa",
    "print_ticket_page" => "Bilet çap etmek",
    "refund_page" => "Yzyna gaýtarmak",
    "seats_page" => "Ýerleri saýlamak",

    // Trip
    "one_way" => "Bir tarap",
    "round_trip" => "Gidiş we gaýdyş",
    "search" => "Gözle",

    // Passenger
    "pnrs_count" => "Ýolagçylaryň sany",
    "pnrs_max" => "Ýolagçylaryň iň köp sany 9",
    "adult" => "Uly adam (10+ ýaş)",
    "child" => "Çaga (5-10 ýaş)",

    // Days of week (2-letter)
    "day_mon" => "Du",
    "day_tue" => "Si",
    "day_wed" => "Ça",
    "day_thu" => "Pe",
    "day_fri" => "An",
    "day_sat" => "Şe",
    "day_sun" => "Ýe",

    // Month names
    "month_1" => "Ýanwar",
    "month_2" => "Fewral",
    "month_3" => "Mart",
    "month_4" => "Aprel",
    "month_5" => "Maý",
    "month_6" => "Iýun",
    "month_7" => "Iýul",
    "month_8" => "Awgust",
    "month_9" => "Sentýabr",
    "month_10" => "Oktýabr",
    "month_11" => "Noýabr",
    "month_12" => "Dekabr",

    // Stations
    "most_popular_places" => "Iň meşhur ýerler",
    "find_by_letters" => "Harplar boýunça hem tapyp bilersiňiz",
    "from" => "Nireden",
    "to" => "Nirä",
    "stations_fetch_error" => "Stansiýalary alyp bolmady",

    // Notifications
    "select_source" => "Ugradylýan stansiýany saýlaň",
    "select_destination" => "Barylýan stansiýany saýlaň",

    // Trips
    "trips_not_found" => "Gatnawlar tapylmady",
    "trips_fetch_error" => "Gatnawlary alyp bolmady",
    "departure" => "Ugraýan wagty",
    "arrival" => "Barýan wagty",
    "price" => "Bahasy",
    "back" => "Yza",
    "km" => "km",
    "hour_short" => "sag",
    "min_short" => "min",

    // Print ticket
    "enter_booking_number" => "Bron belgiňizi giriziň",
    "print" => "Çap etmek",
};
