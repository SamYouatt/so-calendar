use chrono::Datelike;

pub fn days_in_month(month_date_time: impl Datelike) -> u32 {
    let month = month_date_time.month();

    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if month_date_time.year() % 4 == 0 {
                29
            } else {
                28
            }
        }
        _ => panic!("{month} is not a valid month number"),
    }
}
