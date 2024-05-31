use color_eyre::eyre::Result;

use crate::configuration::Application;

use super::account_signin_task::Account;

pub async fn store_account(account: Account, application: &Application) -> Result<i64> {
    let expiry_as_string = account.expiry.to_rfc3339();

    let account_id = sqlx::query!(
        "INSERT INTO accounts (email, access_token, refresh_token, expires_at) VALUES ($1, $2, $3, $4) RETURNING id",
        account.email, account.access_token, account.refresh_token, expiry_as_string
        ).fetch_one(&application.db).await?.id;

    Ok(account_id)
}
