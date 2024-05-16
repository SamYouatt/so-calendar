use chrono::{DateTime, Duration, Utc};
use eyre::Context;
use oauth2::{basic::BasicClient, reqwest::http_client, RefreshToken, TokenResponse};
use reqwest::blocking::RequestBuilder;
use rusqlite::{Connection, Row};
use thiserror::Error;

pub struct GoogleOAuthClient<'a> {
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

impl<'a> GoogleOAuthClient<'a> {
    pub fn new(db: &'a Connection, oauth_client: &'a BasicClient) -> Self {
        GoogleOAuthClient { db, oauth_client }
    }

    pub fn send(
        &self,
        account_email: String,
        request_builder: RequestBuilder,
    ) -> Result<reqwest::blocking::Response, OAuthHttpClientError> {
        let mut statement = self
            .db
            .prepare("SELECT access_token, refresh_token, expires_at FROM accounts WHERE email = ?1 LIMIT 1")
            .wrap_err("Error preparing query to read account tokens")?;
        let mut query_rows = statement.query([&account_email])?;

        let token_details = match query_rows.next()? {
            Some(row) => StoredTokenDetails::from_row(row)?,
            None => return Err(OAuthHttpClientError::NoAccount(account_email)),
        };

        let desired_expiration_time = Utc::now() + Duration::minutes(10);

        let access_token = if token_details.expires_at < desired_expiration_time {
            let refresh_token = RefreshToken::new(token_details.refresh_token);
            let token_response = self
                .oauth_client
                .exchange_refresh_token(&refresh_token)
                .request(http_client)
                .wrap_err("Failed to exchange refresh token")?;

            let new_access_token = token_response.access_token().secret().to_owned();
            let new_expiry = Utc::now()
                + token_response
                    .expires_in()
                    .unwrap_or(std::time::Duration::from_secs(3600));

            upsert_access_token_details(&self.db, &new_access_token, new_expiry, &account_email)?;

            new_access_token
        } else {
            token_details.access_token
        };

        // Step 4: store the new access token and expiration date if there was one

        let result = request_builder.bearer_auth(access_token).send()?;
        Ok(result)
    }
}

fn upsert_access_token_details(
    db: &Connection,
    access_token: &str,
    expires_at: DateTime<Utc>,
    account_email: &str,
) -> Result<(), OAuthHttpClientError> {
    db.execute(
        "UPDATE accounts SET access_token = ?1, expires_at = ?2 WHERE email = ?3",
        [access_token, &expires_at.to_rfc3339(), account_email],
    )?;

    Ok(())
}
