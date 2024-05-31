use chrono::{DateTime, Duration, Utc};
use eyre::Context;
use oauth2::{basic::BasicClient, reqwest::http_client, RefreshToken, TokenResponse};
use reqwest::RequestBuilder;
use sqlx::{query, SqlitePool};
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct GoogleOAuthClient {
    db: SqlitePool,
    oauth_client: BasicClient,
}

#[derive(Error, Debug)]
pub enum OAuthHttpClientError {
    #[error("No matching account found: {0}")]
    NoAccount(String),
    #[error("Reqwest error encountered")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error encountered during database operations")]
    DatabaseError(#[from] sqlx::Error),
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

impl GoogleOAuthClient {
    pub fn new(db: SqlitePool, oauth_client: BasicClient) -> Self {
        GoogleOAuthClient { db, oauth_client }
    }

    pub async fn send(
        &self,
        account_id: i64,
        request_builder: RequestBuilder,
    ) -> Result<reqwest::Response, OAuthHttpClientError> {
        let account_id_string = account_id.to_string();

        let token_details = match query!(
            "SELECT access_token, refresh_token, expires_at FROM accounts WHERE id = $1 LIMIT 1",
            account_id_string
        )
        .fetch_one(&self.db)
        .await
        {
            Ok(row) => {
                let access_token = row.access_token;
                let refresh_token = row.refresh_token;
                let expires_at = row.expires_at;
                let expires_at = DateTime::parse_from_rfc3339(&expires_at)?.into();

                StoredTokenDetails {
                    access_token,
                    refresh_token,
                    expires_at,
                }
            }
            Err(sqlx::Error::RowNotFound) => {
                return Err(OAuthHttpClientError::NoAccount(account_id.to_string()))
            }
            Err(e) => return Err(e.into()),
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

            upsert_access_token_details(&self.db, &new_access_token, new_expiry, account_id).await?;

            new_access_token
        } else {
            token_details.access_token
        };

        let result = request_builder.bearer_auth(access_token).send().await?;
        Ok(result)
    }
}

async fn upsert_access_token_details(
    db: &SqlitePool,
    access_token: &str,
    expires_at: DateTime<Utc>,
    account_id: i64,
) -> Result<(), OAuthHttpClientError> {
    let expires_at_string = expires_at.to_rfc3339();
    let account_id = account_id.to_string();

    query!(
        "UPDATE accounts SET access_token = $1, expires_at = $2 WHERE id = $3",
        access_token,
        expires_at_string,
        account_id,
    )
    .execute(db)
    .await?;

    Ok(())
}
