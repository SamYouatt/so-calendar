// A special wrapper around reqwest that has knowledge of authentication token
// It should:
// - Include auth token as bearer auth on every request made via the client
// - Before making the request it should check if the token has expired or is close to expiry
// - If it is close to expiry it should first refresh the authentication token and store it

use eyre::Context;
use reqwest::blocking::{Client, RequestBuilder};
use rusqlite::Connection;
use thiserror::Error;

pub struct OAuthHttpClient<'a> {
    // TODO: some stuff about the database connection
    db: &'a Connection,
}

#[derive(Error, Debug)]
pub enum OAuthHttpClientError {
    #[error("No matching account found: {0}")]
    NoAccount(String),
    #[error("Reqwest error encountered")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error encountered during database operations")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Unexpected error performing request")]
    UnexpectedError(#[from] color_eyre::eyre::Error),
}

impl<'a> OAuthHttpClient<'a> {
    pub fn new(db: &'a Connection) -> Self {
        OAuthHttpClient { db }
    }

    pub fn send(
        &self,
        account_email: String,
        request_builder: RequestBuilder,
    ) -> Result<reqwest::blocking::Response, OAuthHttpClientError> {
        // Step 1: check for the authentication token
        //      - check if account id exists in table
        //      X - Return error saying not signed in, will definitely be used for control flow
        let mut statement = self
            .db
            .prepare("SELECT auth_token FROM accounts WHERE email = ?1 LIMIT 1")
            .wrap_err("Error preparing query to read account tokens")?;

        let mut auth_rows = statement
            .query([&account_email])
            .wrap_err("Error querying account tokens")?;

        // TODO: query map here when using more fields
        let auth_token: String = match auth_rows
            .next()
            .wrap_err("Error reading rows from query result")?
        {
            Some(token_row) => token_row.get(0).wrap_err("Error reading returned row")?,
            None => return Err(OAuthHttpClientError::NoAccount(account_email)),
        };

        // Step 2: check the expiry on the authentication token, if less than 10 mins left, refresh
        //      - read expiry date from table and check how long left
        //      X - Not much can be done here so opaque should be fine

        // Step 2.5: refresh flow
        //      - Make the refresh token request
        //      - Store the new token for that account
        //      X - Not much can be done here so opaque is fine

        // Step 3:
        //      - Attacth the auth token to the request as bearer auth
        //      X - Return this response as is - leave it for caller to decide what to do
        let result = request_builder.bearer_auth("blah").send()?;
        Ok(result)
    }
}
