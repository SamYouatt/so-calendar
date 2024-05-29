use color_eyre::eyre::Result;
use sqlx::query;
use uuid::Uuid;

use crate::configuration::Application;

use super::account_signin_task::Account;

pub async fn store_account(account: Account, application: &Application) -> Result<Uuid> {
    let expiry_as_string = account.expiry.to_rfc3339();
    let account_id = Uuid::new_v4();
    let account_id_string = account_id.to_string();

    query!(
        "INSERT INTO accounts (id, email, access_token, refresh_token, expires_at) VALUES ($1, $2, $3, $4, $5)",
        account_id_string, account.email, account.access_token, account.refresh_token, expiry_as_string
        ).execute(&application.db).await?;

    Ok(account_id)
}
