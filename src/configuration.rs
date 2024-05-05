use std::{fs, path::PathBuf};

use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use rusqlite::Connection;

pub enum ApplicationError {
    FailedToCreateAccounts,
}

pub struct Application {
    pub data_dir: PathBuf,
    pub db_path: PathBuf,
    pub oauth_client: BasicClient,
}

impl Application {
    pub fn new() -> Self {
        let data_dir = dirs_next::data_dir()
            .expect("Unable to find data directory")
            .join("so-calendar");

        let db_path = data_dir.join("app.sqlite");

        let oauth_client = configure_oauth_client();

        Self { data_dir, db_path, oauth_client }
    }

    pub fn setup(&self) -> Result<(), ApplicationError> {
        // Create app data directory
        fs::create_dir_all(&self.data_dir).expect("Failed to create data directory");

        // Create required tables
        let db = Connection::open(&self.db_path).unwrap();

        db.execute(
            "CREATE TABLE IF NOT EXISTS accounts(
                    id integer PRIMARY KEY,
                    email text NOT NULL UNIQUE,
                    access_token text NOT NULL,
                    refresh_token text NOT NULL,
                    expires_at text NOT NULL
                )",
            [],
        )
        .map_err(|e| {
            println!("Error creating accounts table: {:?}", e);
            ApplicationError::FailedToCreateAccounts
        })?;

        Ok(())
    }
}

pub fn configure_oauth_client() -> BasicClient {
    let auth_url_raw = String::from("https://accounts.google.com/o/oauth2/v2/auth");
    let auth_url = AuthUrl::new(auth_url_raw).expect("Invalid auth endpoint");

    let token_url_raw = String::from("https://www.googleapis.com/oauth2/v3/token");
    let token_url = TokenUrl::new(token_url_raw).expect("Invalid token endpoint");

    BasicClient::new(
        ClientId::new(
            "357015344564-7rf7b47n7add82k2t3hajfhq2pklthen.apps.googleusercontent.com".into(),
        ),
        Some(ClientSecret::new(
            "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".into(),
        )),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:42069/auth/redirect".into()).unwrap())
}
