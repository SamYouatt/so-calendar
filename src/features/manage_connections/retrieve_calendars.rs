use color_eyre::eyre::Result;
use sqlx::SqlitePool;

use super::Calendar;

pub async fn retrieve_calendars(db: &SqlitePool) -> Result<Vec<Calendar>> {
    let calendars: Vec<Calendar> = sqlx::query_as!(Calendar, r#"SELECT id as "id:uuid::Uuid", account_id as "account_id: uuid::Uuid", title, description FROM calendars"#)
        .fetch_all(db)
        .await?;

    Ok(calendars)
}
