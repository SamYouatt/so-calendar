use chrono::{DateTime, NaiveDate, Utc};

pub mod days_view_state;
pub(crate) mod handle_days_view_message;
pub(crate) mod retrieve_calendars;
pub(crate) mod view;
pub(crate) mod deserialise_event_response;

#[derive(Debug)]
pub struct Event {
    id: String,
    title: String,
    description: Option<String>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

#[derive(Debug)]
pub struct DayEvent {
    id: String,
    title: String,
    description: Option<String>,
    date: NaiveDate,
}

#[derive(Debug)]
pub struct CancelledEvent {
    id: String,
}
