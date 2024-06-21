use color_eyre::eyre::Result;
use sqlx::SqlitePool;

use super::Calendar;

pub async fn retrieve_calendars(db: &SqlitePool) -> Result<Vec<Calendar>> {
    let calendars: Vec<Calendar> = sqlx::query_as!(
        Calendar,
        r#"SELECT id, account_id, title, description FROM calendars"#
    )
    .fetch_all(db)
    .await?;

    Ok(calendars)
}
