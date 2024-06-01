use color_eyre::eyre::Result;
use sqlx::SqlitePool;

pub struct Calendar {
    pub calendar_id: String,
    pub account_id: i64,
}

pub async fn retrieve_calendars(db: &SqlitePool) -> Result<Vec<Calendar>> {
    let calendars: Vec<Calendar> = sqlx::query_as!(Calendar, "SELECT calendar_id, account_id FROM calendars")
        .fetch_all(db)
        .await?;

    Ok(calendars)
}
