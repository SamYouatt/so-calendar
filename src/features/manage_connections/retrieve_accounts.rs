use color_eyre::eyre::Result;
use eyre::Context;
use sqlx::query_as;
use std::fmt::Display;

use crate::configuration::Application;

use super::Account;

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email)
    }
}

pub async fn retrieve_accounts(application: &Application) -> Result<Vec<Account>> {
    let accounts = query_as!(Account, "SELECT email FROM accounts")
        .fetch_all(&application.db)
        .await
        .wrap_err("Error while retrieving stored accounts")?;

    Ok(accounts)
}
