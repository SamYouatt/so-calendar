use chrono::{Duration, Local, NaiveTime};
use color_eyre::eyre::Result;

use crate::{
    features::{
        days_view::{days_view_state::DaysViewState, retrieve_calendars::retrieve_calendars},
        oauth_http_client::GoogleOAuthClient,
    },
    tui::model::{CurrentState, Model},
};

use super::{
    deserialise_event_response::deserialise_event_list_response, retrieve_calendars::Calendar,
    DayEvent, Event,
};

pub async fn handle_load_days_view(model: &mut Model) -> Result<()> {
    let calendars = retrieve_calendars(&model.application.db).await?;

    let mut all_events = vec![];
    let mut all_day_events = vec![];
    for calendar in calendars.into_iter() {
        // TODO: some of these might fail
        let (mut events, mut day_events) =
            retrieve_calendar_events(calendar, &model.application.google_client).await?;
        all_events.append(&mut events);
        all_day_events.append(&mut day_events);
    }

    model.current_state = CurrentState::DaysView(DaysViewState {
        events: all_events,
        day_events: all_day_events,
    });
    Ok(())
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
