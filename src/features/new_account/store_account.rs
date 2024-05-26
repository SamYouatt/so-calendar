use color_eyre::eyre::Result;
use sqlx::query;

use crate::configuration::Application;

use super::handle_new_account::Account;

pub async fn store_account(account: Account, application: &Application) -> Result<()> {
    let expiry_as_string = account.expiry.to_rfc3339();

    query!(
        "INSERT INTO accounts (email, access_token, refresh_token, expires_at) VALUES ($1, $2, $3, $4)",
        account.email, account.access_token, account.refresh_token, expiry_as_string
        ).execute(&application.db).await?;

    Ok(())
}
