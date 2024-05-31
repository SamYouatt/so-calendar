use color_eyre::eyre::Result;
use eyre::Context;
use sqlx::{query_as, SqlitePool};
use std::fmt::Display;

use super::Account;

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email)
    }
}

pub async fn retrieve_accounts(db: &SqlitePool) -> Result<Vec<Account>> {
    let accounts = query_as!(Account, r#"SELECT id as "id: uuid::Uuid", email FROM accounts"#)
        .fetch_all(db)
        .await
        .wrap_err("Error while retrieving stored accounts")?;

    Ok(accounts)
}
