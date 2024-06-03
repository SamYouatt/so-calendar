use chrono::{DateTime, NaiveDate, Utc};
use color_eyre::eyre::Result;
use reqwest::Response;
use serde::Deserialize;

use super::{DayEvent, Event};

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

impl From<&ConfirmedEventResource> for Event {
    fn from(event_resource: &ConfirmedEventResource) -> Self {
        Self {
            id: event_resource.id.to_owned(),
            title: event_resource.summary.to_owned(),
            description: event_resource.description.to_owned(),
            start_time: event_resource.start.date_time,
            end_time: event_resource.end.date_time,
        }
    }
}

impl From<&ConfirmedDayEventResource> for DayEvent {
    fn from(event_resource: &ConfirmedDayEventResource) -> Self {
        Self {
            id: event_resource.id.to_owned(),
            title: event_resource.summary.to_owned(),
            description: event_resource.description.to_owned(),
            date: event_resource.start.date,
        }
    }
}

pub async fn deserialise_event_list_response(
    response: Response,
) -> Result<(Vec<Event>, Vec<DayEvent>)> {
    let event_list: EventListResponse = response.json().await?;

    let events: Vec<Event> = event_list
        .items
        .iter()
        .filter_map(|event| match event {
            EventResource::Confirmed(ConfirmedEvent::Event(x)) => Some(x),
            _ => None,
        })
        .map(|event| Event::from(event))
        .collect();

    let day_events: Vec<DayEvent> = event_list
        .items
        .iter()
        .filter_map(|event| match event {
            EventResource::Confirmed(ConfirmedEvent::DayEvent(x)) => Some(x),
            _ => None,
        })
        .map(|event| DayEvent::from(event))
        .collect();

    Ok((events, day_events))
}

#[cfg(test)]
mod test {
    use crate::features::days_view::DayEvent;

    use super::deserialise_event_list_response;
    use reqwest::Response;

    #[tokio::test]
    async fn parse_confirmed_event() {
        // Arrange
        let raw_json = r#"
{
    "kind": "calendar#events",
    "etag": "etag",
    "summary": "test@test.com",
    "description": "",
    "updated": "2024-06-02T14:16:07.916Z",
    "timeZone": "Europe/London",
    "accessRole": "owner",
    "defaultReminders": [
        {
            "method": "popup",
            "minutes": 10
        }
    ],
    "nextSyncToken": "sync_token",
    "items": [
        {
            "kind": "calendar#event",
            "etag": "etag",
            "id": "test_id",
            "status": "confirmed",
            "htmlLink": "link",
            "created": "2022-12-20T11:28:24.000Z",
            "updated": "2024-01-15T16:48:23.841Z",
            "summary": "Super cool test event",
            "colorId": "8",
            "creator": {
                "email": "test@test.com",
                "self": true
            },
            "organizer": {
                "email": "test@test.com",
                "self": true
            },
            "start": {
                "dateTime": "2022-12-21T12:00:00Z",
                "timeZone": "Europe/London"
            },
            "end": {
                "dateTime": "2022-12-21T13:30:00Z",
                "timeZone": "Europe/London"
            },
            "visibility": "private",
            "iCalUID": "blongo",
            "sequence": 1,
            "guestsCanModify": true,
            "reminders": {
                "useDefault": false
            },
            "eventType": "default"
        }
    ]
}
        "#;
        let response = construct_response(raw_json);

        // Act
        let (events, day_events) = deserialise_event_list_response(response).await.unwrap();

        // Assert
        assert_eq!(events[0].title, "Super cool test event");
        assert_eq!(events[0].id, "test_id");
        assert_eq!(events[0].description, None);
        assert_eq!(day_events.len(), 0);
    }

    #[tokio::test]
    async fn ignores_cancelled_events() {
        // Arrange
        let raw_json = r#"
{
  "kind": "calendar#events",
  "etag": "etag",
  "summary": "test@test.com",
  "description": "",
  "updated": "2024-06-02T14:16:07.916Z",
  "timeZone": "Europe/London",
  "accessRole": "owner",
  "defaultReminders": [
    {
      "method": "popup",
      "minutes": 10
    }
  ],
  "nextSyncToken": "CODj9tyNvYYDEODj9tyNvYYDGAUgkpaKsAIokpaKsAI=",
  "items": [
    {
      "kind": "calendar#event",
      "etag": "etag",
      "id": "test_id",
      "status": "cancelled"
    }
  ]
}        "#;
        let response = construct_response(raw_json);

        // Act
        let (events, day_events) = deserialise_event_list_response(response).await.unwrap();

        // Assert
        assert_eq!(events.len(), 0);
        assert_eq!(day_events.len(), 0);
    }

    #[tokio::test]
    async fn parse_confired_day_events() {
        // Arrange
        let raw_json = r#"
{
  "kind": "calendar#events",
  "etag": "etag",
  "summary": "test@test.com",
  "description": "",
  "updated": "2024-06-02T14:16:07.916Z",
  "timeZone": "Europe/London",
  "accessRole": "owner",
  "defaultReminders": [
    {
      "method": "popup",
      "minutes": 10
    }
  ],
  "nextSyncToken": "CODj9tyNvYYDEODj9tyNvYYDGAUgkpaKsAIokpaKsAI=",
  "items": [
    {
      "kind": "calendar#event",
      "etag": "etag",
      "id": "test_id",
      "status": "confirmed",
      "htmlLink": "test_link",
      "created": "2023-06-15T13:04:50.000Z",
      "updated": "2024-01-25T14:21:25.869Z",
      "summary": "Super cool all day test event",
      "creator": {
        "email": "sam.youatt@gearset.com",
        "self": true
      },
      "organizer": {
        "email": "test@test.com",
        "self": true
      },
      "start": {
        "date": "2023-06-19"
      },
      "end": {
        "date": "2023-06-20"
      },
      "recurrence": [
        "RRULE:FREQ=WEEKLY;BYDAY=MO"
      ],
      "transparency": "transparent",
      "visibility": "public",
      "iCalUID": "ical@google.com",
      "sequence": 0,
      "reminders": {
        "useDefault": false
      },
      "workingLocationProperties": {
        "type": "homeOffice",
        "homeOffice": {}
      },
      "eventType": "workingLocation"
    }
  ]
}     
        "#;
        let response = construct_response(raw_json);

        // Act
        let (events, day_events) = deserialise_event_list_response(response).await.unwrap();

        // Assert
        assert_eq!(day_events[0].title, "Super cool all day test event");
        assert_eq!(day_events[0].id, "test_id");
        assert_eq!(day_events[0].description, None);
        assert_eq!(events.len(), 0);
    }

    fn construct_response(body: &str) -> Response {
        let response = http::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(body.to_string())
            .unwrap();

        Response::from(response)
    }
}
