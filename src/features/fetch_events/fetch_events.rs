use chrono::{Duration, Local, NaiveDate, NaiveTime};
use color_eyre::eyre::Result;
use sqlx::SqlitePool;

use crate::{
    domain::{
        calendar::Calendar,
        events::{DayEvent, Event},
    },
    features::oauth_http_client::GoogleOAuthClient,
    tui::model::Model,
};

use super::deserialise_event_response::deserialise_event_list_response;

/// Begin a fetch of calendar events which will send a message with
/// the results, or an error message
/// start_date: inclusive start date at midnight
/// end_date: exclusive start date - events up to this date at midnight
pub fn run_fetch_events_task(start_date: NaiveDate, end_date: NaiveDate, model: &Model) {
    let db = model.application.db.clone();
    let google_client = model.application.google_client.clone();

    tokio::spawn(async move {
        match fetch_events(start_date, end_date, db, google_client).await {
            // Send finished event with payload
            Ok(_) => todo!(),
            // Create error message event with user facing error
            Err(_) => todo!(),
        }
    });
}

async fn fetch_events(
    start_date: NaiveDate,
    end_date: NaiveDate,
    db: SqlitePool,
    google_client: GoogleOAuthClient,
) -> Result<(Vec<Event>, Vec<DayEvent>)> {
    let calendars = retrieve_calendars(&db).await?;

    let mut all_events = vec![];
    let mut all_day_events = vec![];
    for calendar in calendars.into_iter() {
        // TODO: some of these might fail
        let (mut events, mut day_events) =
            retrieve_calendar_events(calendar, &google_client).await?;
        all_events.append(&mut events);
        all_day_events.append(&mut day_events);
    }

    todo!()
}

async fn retrieve_calendars(db: &SqlitePool) -> Result<Vec<Calendar>> {
    let calendars: Vec<Calendar> =
        sqlx::query_as!(Calendar, "SELECT calendar_id, account_id FROM calendars")
            .fetch_all(db)
            .await?;

    Ok(calendars)
}

async fn retrieve_calendar_events(
    calendar: Calendar,
    google_client: &GoogleOAuthClient,
) -> Result<(Vec<Event>, Vec<DayEvent>)> {
    let now = Local::now();
    let today_midnight = now.with_time(NaiveTime::MIN).unwrap();
    let tomorrow_midnight = today_midnight + Duration::days(2);

    let events_starting_from = today_midnight.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let events_ending_at = tomorrow_midnight.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    let events_list_url = format!(
        "https://www.googleapis.com/calendar/v3/calendars/{}/events",
        urlencoding::encode(&calendar.calendar_id)
    );

    let http_client = reqwest::Client::new();
    let calendar_list_request = http_client.get(events_list_url.to_string()).query(&[
        ("timeMin", events_starting_from.as_str()),
        ("timeMax", events_ending_at.as_str()),
    ]);

    let response = google_client
        .send(calendar.account_id, calendar_list_request)
        .await?;

    let events = deserialise_event_list_response(response).await?;

    Ok(events)
}
