use chrono::{Duration, Local, NaiveTime};
use color_eyre::eyre::Result;

use crate::{
    features::{
        days_view::{days_view_state::DaysViewState, retrieve_calendars::retrieve_calendars},
        oauth_http_client::GoogleOAuthClient,
    },
    tui::model::{CurrentState, Model},
};

use super::{retrieve_calendars::Calendar, Event, deserialise_event_response::EventListResponse};

pub async fn handle_load_days_view(model: &mut Model) -> Result<()> {
    let calendars = retrieve_calendars(&model.application.db).await?;

    let mut all_events = vec![];
    for calendar in calendars.into_iter() {
        // TODO: some of these might fail
        let mut events = retrieve_calendar_events(calendar, &model.application.google_client).await?;
        all_events.append(&mut events);
    }

    println!("{:?}", all_events);
    model.current_state = CurrentState::DaysView(DaysViewState { events: all_events });
    Ok(())
}

async fn retrieve_calendar_events(
    calendar: Calendar,
    google_client: &GoogleOAuthClient,
) -> Result<Vec<Event>> {
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

    //println!("{:?}", response.bytes().await.unwrap());
    //Ok(vec![])

    // TODO: handle non ok responses

    let event_list: EventListResponse = response.json().await?;
    println!("{:#?}", event_list);
    Ok(vec![])
    //let events: Vec<Event> = event_list
    //    .items
    //    .into_iter()
    //    .map(|event| Event::from(event))
    //    .collect();
    //
    //Ok(events)
}

#[cfg(test)]
mod test {
    use crate::{
        configuration::Application,
        features::days_view::{
            handle_days_view_message::retrieve_calendar_events, retrieve_calendars::Calendar,
        },
    };

    #[tokio::test]
    async fn test_google_events() {
        let application = Application::setup().await.unwrap();

        let calendar = Calendar {
            account_id: 2,
            calendar_id: "sam.youatt@gearset.com".to_string(),
        };

        let result = retrieve_calendar_events(calendar, &application.google_client)
            .await
            .unwrap();

        println!("{:#?}", result);

        assert!(false);
    }
}
