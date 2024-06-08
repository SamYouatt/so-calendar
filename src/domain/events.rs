use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Debug)]
pub struct DayEvent {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub date: NaiveDate,
}
