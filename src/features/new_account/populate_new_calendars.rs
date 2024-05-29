use color_eyre::eyre::Result;
use serde::Deserialize;

use crate::configuration::Application;

// https://developers.google.com/calendar/api/v3/reference/calendarList#resource
#[derive(Deserialize, Debug)]
struct CalendarListResponse {
    items: Vec<CalendarResource>,
}

// https://developers.google.com/calendar/api/v3/reference/calendarList#resource
#[derive(Deserialize, Debug)]
struct CalendarResource {
    id: String,
    primary: Option<bool>,
    summary: String,
    description: Option<String>,
}

struct Calendar {
    id: String,
    primary_calendar: bool,
    title: String,
    description: Option<String>,
}

impl From<CalendarResource> for Calendar {
    fn from(calendar: CalendarResource) -> Self {
        Self {
            id: calendar.id,
            primary_calendar: calendar.primary.unwrap_or(false),
            title: calendar.summary,
            description: calendar.description,
        }
    }
}

pub async fn populate_new_calendars(email: String, application: &Application) -> Result<()> {
    let http_client = reqwest::Client::new();
    let calendar_list_request =
        http_client.get("https://www.googleapis.com/calendar/v3/users/me/calendarList");

    let response = application
        .google_client
        .send(email, calendar_list_request)
        .await?;

    let calendar_list: CalendarListResponse = response.json().await?;
    let calendars: Vec<Calendar> = calendar_list
        .items
        .into_iter()
        .map(|cal| Calendar::from(cal))
        .collect();

    store_calendars(&calendars, application).await
}

async fn store_calendars(calendars: &[Calendar], application: &Application) -> Result<()> {
    todo!()
}
