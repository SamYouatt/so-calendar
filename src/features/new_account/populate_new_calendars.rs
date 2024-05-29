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

pub async fn populate_new_calendars(email: String, application: &Application) -> Result<()> {
    let http_client = reqwest::Client::new();
    let calendar_list_request =
        http_client.get("https://www.googleapis.com/calendar/v3/users/me/calendarList");
    let response = application.google_client.send(email, calendar_list_request).await?;

    let calendar_list: CalendarListResponse = response.json().await?;

    todo!()
}
