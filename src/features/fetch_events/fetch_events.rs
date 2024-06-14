use chrono::{DateTime, Local};
use color_eyre::eyre::Result;
use eyre::Context;
use sqlx::SqlitePool;

use crate::{
    domain::{
        calendar::Calendar,
        events::{DayEvent, Event},
    },
    features::oauth_http_client::GoogleOAuthClient,
    tui::model::{Message, Model},
};

use super::deserialise_event_response::deserialise_event_list_response;

/// Begin a fetch of calendar events which will send a message with
/// the results, or an error message
/// start_time: start time to fetch events
/// end_time: exclusive end time to fetch events
pub fn run_fetch_events_task(
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    model: &Model,
) {
    let db = model.application.db.clone();
    let google_client = model.application.google_client.clone();
    let message_channel = model.message_channel.clone();

    tokio::spawn(async move {
        match fetch_events(start_time, end_time, db, google_client).await {
            Ok(result) => message_channel
                .send(Message::EventsReady(result.0, result.1))
                .expect("Message channel should never be closed"),
            // Create error message event with user facing error
            Err(_) => todo!(),
        }
    });
}

async fn fetch_events(
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    db: SqlitePool,
    google_client: GoogleOAuthClient,
) -> Result<(Vec<Event>, Vec<DayEvent>)> {
    let calendars = retrieve_calendars(&db).await?;

    let mut all_events = vec![];
    let mut all_day_events = vec![];
    for calendar in calendars.into_iter() {
        // TODO: make these all run in parallel
        // TODO: some of these might fail
        let (mut events, mut day_events) =
            retrieve_calendar_events(start_time, end_time, calendar, &google_client).await?;
        all_events.append(&mut events);
        all_day_events.append(&mut day_events);
    }

    Ok((all_events, all_day_events))
}

async fn retrieve_calendars(db: &SqlitePool) -> Result<Vec<Calendar>> {
    sqlx::query_as!(Calendar, "SELECT calendar_id, account_id FROM calendars")
        .fetch_all(db)
        .await
        .wrap_err("error while retrieving stored calendars")
}

async fn retrieve_calendar_events(
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    calendar: Calendar,
    google_client: &GoogleOAuthClient,
) -> Result<(Vec<Event>, Vec<DayEvent>)> {
    let events_from = start_time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let events_until = end_time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    let events_list_url = format!(
        "https://www.googleapis.com/calendar/v3/calendars/{}/events",
        urlencoding::encode(&calendar.calendar_id)
    );

    let http_client = reqwest::Client::new();
    let calendar_list_request = http_client.get(events_list_url.to_string()).query(&[
        ("timeMin", events_from.as_str()),
        ("timeMax", events_until.as_str()),
    ]);

    let response = google_client
        .send(calendar.account_id, calendar_list_request)
        .await?;

    let events = deserialise_event_list_response(response).await?;

    Ok(events)
}
