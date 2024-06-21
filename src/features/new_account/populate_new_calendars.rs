use color_eyre::eyre::Result;
use eyre::eyre;
use futures::future::join_all;
use serde::Deserialize;
use sqlx::SqlitePool;

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

pub async fn populate_new_calendars(account_id: i64, application: &Application) -> Result<()> {
    let http_client = reqwest::Client::new();
    let calendar_list_request =
        http_client.get("https://www.googleapis.com/calendar/v3/users/me/calendarList");

    let response = application
        .google_client
        .send(account_id, calendar_list_request)
        .await?;

    let calendar_list: CalendarListResponse = response.json().await?;
    let calendars: Vec<Calendar> = calendar_list
        .items
        .into_iter()
        .map(|cal| Calendar::from(cal))
        .collect();

    let _ = store_calendars(calendars, account_id, application)
        .await
        .unwrap();

    Ok(())
}

async fn store_calendars(
    calendars: Vec<Calendar>,
    account_id: i64,
    application: &Application,
) -> Result<()> {
    let store_row_queries: Vec<_> = calendars
        .into_iter()
        .map(|calendar| store_row(calendar, account_id, &application.db))
        .collect();

    let results = join_all(store_row_queries).await;

    if results.iter().any(|res| res.is_err()) {
        let error_results = results.iter().filter(|res| res.is_err());
        return Err(eyre!(
            "Error while upserting new calendars: {:?}",
            error_results
        ));
    }

    Ok(())
}

async fn store_row(calendar: Calendar, account_id: i64, db: &SqlitePool) -> Result<()> {
    let _ = sqlx::query!(
        "INSERT INTO calendars
        (calendar_id, account_id, title, description, primary_calendar)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (calendar_id)
        DO UPDATE SET title=excluded.title,
            description=excluded.description,
            primary_calendar=excluded.primary_calendar",
        calendar.id,
        account_id,
        calendar.title,
        calendar.description,
        calendar.primary_calendar
    )
    .execute(db)
    .await?;

    Ok(())
}
