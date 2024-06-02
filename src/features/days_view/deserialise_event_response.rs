use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

use super::Event;

#[derive(Deserialize, Debug)]
pub struct EventListResponse {
    items: Vec<EventResource>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "status")]
pub enum EventResource {
    #[serde(rename = "confirmed")]
    Confirmed(ConfirmedEvent),
    #[serde(rename = "tentative")]
    Tentative(ConfirmedEvent),
    #[serde(rename = "cancelled")]
    Cancelled(CancelledEventResource),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ConfirmedEvent {
    Event(ConfirmedEventResource),
    DayEvent(ConfirmedDayEventResource),
}

#[derive(Deserialize, Debug)]
struct ConfirmedEventResource {
    id: String,
    summary: String,
    description: Option<String>,
    start: DateObject,
    end: DateObject,
}

#[derive(Deserialize, Debug)]
struct ConfirmedDayEventResource {
    id: String,
    summary: String,
    description: Option<String>,
    start: DayDateObject,
    end: DayDateObject,
}

#[derive(Deserialize, Debug)]
pub struct CancelledEventResource {
    id: String,
}

#[derive(Deserialize, Debug)]
struct DateObject {
    #[serde(rename = "dateTime")]
    date_time: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
struct DayDateObject {
    date: NaiveDate,
}

impl From<ConfirmedEventResource> for Event {
    fn from(event_resource: ConfirmedEventResource) -> Self {
        Event {
            id: event_resource.id,
            title: event_resource.summary,
            description: event_resource.description,
            start_time: event_resource.start.date_time,
            end_time: event_resource.end.date_time,
        }
    }
}
