use chrono::{DateTime, Duration, Local, NaiveTime, Utc};
use color_eyre::eyre::Result;
use serde::Deserialize;

use crate::{
    features::{
        days_view::{days_view_state::DaysViewState, retrieve_calendars::retrieve_calendars},
        oauth_http_client::GoogleOAuthClient,
    },
    tui::model::{CurrentState, Model},
};

use super::retrieve_calendars::Calendar;

#[derive(Deserialize)]
struct EventListResponse {
    items: Vec<EventResource>,
}

#[derive(Deserialize)]
struct EventResource {
    summary: String,
    description: Option<String>,
    start: DateObject,
    end: DateObject,
}

#[derive(Deserialize)]
struct DateObject {
    #[serde(rename = "dateTime")]
    date_time: DateTime<Utc>,
}

#[derive(Debug)]
struct Event {
    title: String,
    description: Option<String>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

impl From<EventResource> for Event {
    fn from(event_resource: EventResource) -> Self {
        Event {
            title: event_resource.summary,
            description: event_resource.description,
            start_time: event_resource.start.date_time,
            end_time: event_resource.end.date_time,
        }
    }
}

pub async fn handle_load_days_view(model: &mut Model) -> Result<()> {
    let calendars = retrieve_calendars(&model.application.db).await?;

    for calendar in calendars.into_iter() {
        let events = retrieve_calendar_events(calendar, &model.application.google_client).await?;
        println!("{:?}", events);
    }

    model.current_state = CurrentState::DaysView(DaysViewState {});
    Ok(())
}

async fn retrieve_calendar_events(
    calendar: Calendar,
    google_client: &GoogleOAuthClient,
) -> Result<Vec<Event>> {
    let now = Local::now();
    let today_midnight = now.with_time(NaiveTime::MIN).unwrap();
    let tomorrow_midnight = today_midnight + Duration::days(1);

    let events_starting_from = today_midnight.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let events_ending_at = tomorrow_midnight.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    let events_list_url = format!(
        "https://www.googleapis.com/calendar/v3/calendars/{}/events",
        calendar.calendar_id
    );

    let http_client = reqwest::Client::new();
    let calendar_list_request = http_client.get(events_list_url).query(&[
        ("timeMin", events_starting_from.as_str()),
        ("timeMax", events_ending_at.as_str()),
    ]);

    let response = google_client
        .send(calendar.account_id, calendar_list_request)
        .await?;

    println!("{:?}", response.bytes().await.unwrap());

    //let event_list: EventListResponse = response.json().await?;
    //let events: Vec<Event> = event_list
    //    .items
    //    .into_iter()
    //    .map(|event| Event::from(event))
    //    .collect();
    //
    //Ok(events)

    Ok(vec![])
}
