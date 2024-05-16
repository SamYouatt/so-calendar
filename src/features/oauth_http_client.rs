// A special wrapper around reqwest that has knowledge of authentication token
// It should:
// - Include auth token as bearer auth on every request made via the client
// - Before making the request it should check if the token has expired or is close to expiry
// - If it is close to expiry it should first refresh the authentication token and store it

use chrono::{DateTime, Duration, Utc};
use eyre::Context;
use oauth2::{basic::BasicClient, reqwest::http_client, AccessToken, RefreshToken, TokenResponse};
use reqwest::blocking::RequestBuilder;
use rusqlite::{Connection, Row};
use thiserror::Error;

pub struct OAuthHttpClient<'a> {
    db: &'a Connection,
    oauth_client: &'a BasicClient,
}

#[derive(Error, Debug)]
pub enum OAuthHttpClientError {
    #[error("No matching account found: {0}")]
    NoAccount(String),
    #[error("Reqwest error encountered")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error encountered during database operations")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Invalid stored expiration date")]
    InvalidExpiration(#[from] chrono::ParseError),
    #[error("Unexpected error performing request")]
    UnexpectedError(#[from] color_eyre::eyre::Error),
}

struct StoredTokenDetails {
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

impl StoredTokenDetails {
    fn from_row(row: &Row) -> Result<Self, OAuthHttpClientError> {
        let access_token: String = row.get(0)?;
        let refresh_token: String = row.get(1)?;
        let expires_at: String = row.get(2)?;
        let expires_at = DateTime::parse_from_rfc3339(&expires_at)?.into();

        Ok(StoredTokenDetails {
            access_token,
            refresh_token,
            expires_at,
        })
    }
}

impl<'a> OAuthHttpClient<'a> {
    pub fn new(db: &'a Connection, oauth_client: &'a BasicClient) -> Self {
        OAuthHttpClient { db, oauth_client }
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
            .prepare("SELECT (auth_token, refresh_token, expires_at) FROM accounts WHERE email = ?1 LIMIT 1")
            .wrap_err("Error preparing query to read account tokens")?;

        let mut query_rows = statement.query([&account_email])?;

        let token_details = match query_rows.next()? {
            Some(row) => StoredTokenDetails::from_row(row)?,
            None => return Err(OAuthHttpClientError::NoAccount(account_email)),
        };

        // Step 2: check the expiry on the authentication token, if less than 10 mins left, refresh
        //      - read expiry date from table and check how long left
        //      X - Not much can be done here so opaque should be fine
        let desired_expiration_time = Utc::now() + Duration::minutes(10);

        // Step 2.5: refresh flow
        //      - Make the refresh token request
        //      - Store the new token for that account
        //      X - Not much can be done here so opaque is fine
        let auth_token = if token_details.expires_at < desired_expiration_time {
            let refresh_token = RefreshToken::new(token_details.refresh_token);
            let token_response = self
                .oauth_client
                .exchange_refresh_token(&refresh_token)
                .request(http_client)
                .wrap_err("Failed to exchange refresh token")?;

            token_response.access_token().to_owned()
        } else {
            AccessToken::new(token_details.access_token)
        };

        // Step 3:
        //      - Attacth the auth token to the request as bearer auth
        //      X - Return this response as is - leave it for caller to decide what to do
        let result = request_builder.bearer_auth(auth_token.secret()).send()?;
        Ok(result)
    }
}
