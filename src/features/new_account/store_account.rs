use crate::configuration::Application;

use super::{handle_new_account::Account, NewAccountError};

pub fn store_account(account: Account, application: &Application) -> Result<(), NewAccountError> {
    application.db.execute(
        "INSERT INTO accounts (email, access_token, refresh_token, expires_at) VALUES (?1, ?2, ?3, ?4)",
        [account.email, account.access_token, account.refresh_token, account.expiry.to_rfc3339()],
        )
        .map_err(|e| {
            println!("Error inserting account: {:?}", e);
            NewAccountError::SqliteError
        })?;

    Ok(())
}
