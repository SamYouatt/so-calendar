use color_eyre::eyre::Result;
use eyre::Context;
use sqlx::{query_as, SqlitePool};

use super::Account;

pub async fn retrieve_accounts(db: &SqlitePool) -> Result<Vec<Account>> {
    let accounts = query_as!(Account, r#"SELECT id, email FROM accounts"#)
        .fetch_all(db)
        .await
        .wrap_err("Error while retrieving stored accounts")?;

    Ok(accounts)
}
