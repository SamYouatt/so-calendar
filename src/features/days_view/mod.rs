use chrono::{DateTime, NaiveDate, Utc};

pub mod days_view_state;
pub(crate) mod handle_days_view_message;
pub(crate) mod retrieve_calendars;
pub(crate) mod view;
pub(crate) mod deserialise_event_response;

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
