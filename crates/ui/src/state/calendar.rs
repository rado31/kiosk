use chrono::{Datelike, FixedOffset, NaiveDate, Utc};

pub fn today() -> NaiveDate {
    let tmt = FixedOffset::east_opt(5 * 3600).unwrap();

    Utc::now().with_timezone(&tmt).date_naive()
}

/// Returns 0=Mon, 1=Tue, ..., 6=Sun.
pub fn weekday_index(date: NaiveDate) -> u32 {
    date.weekday().num_days_from_monday()
}

pub fn days_in_month(year: i32, month: u32) -> u32 {
    let next = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };

    next.unwrap()
        .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
        .num_days() as u32
}

pub struct State {
    pub viewed_year: i32,
    pub viewed_month: u32,
    one_way_date: NaiveDate,
    pub round_trip_date: NaiveDate,
}

impl Default for State {
    fn default() -> Self {
        let t = today();

        Self {
            viewed_year: t.year(),
            viewed_month: t.month(),
            one_way_date: t,
            round_trip_date: t,
        }
    }
}

impl State {
    pub fn view_date(&mut self, date: NaiveDate) {
        self.viewed_year = date.year();
        self.viewed_month = date.month();
    }

    pub fn prev_month(&mut self) {
        if self.viewed_month == 1 {
            self.viewed_month = 12;
            self.viewed_year -= 1;

            return;
        }

        self.viewed_month -= 1;
    }

    pub fn next_month(&mut self) {
        if self.viewed_month == 12 {
            self.viewed_month = 1;
            self.viewed_year += 1;

            return;
        }

        self.viewed_month += 1;
    }

    pub fn one_way_date(&self) -> NaiveDate {
        self.one_way_date
    }

    pub fn set_one_way_date(&mut self, date: NaiveDate) {
        self.one_way_date = date;

        if self.round_trip_date <= date {
            self.round_trip_date = date.succ_opt().unwrap_or(date);
        }
    }

    pub fn set_round_trip_date(&mut self, date: NaiveDate) {
        self.round_trip_date = date;
    }
}
