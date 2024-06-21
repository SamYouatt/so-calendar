use color_eyre::eyre::Result;
use dotenv_codegen::dotenv;
use eyre::Context;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::{fs, path::PathBuf};

use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use crate::features::oauth_http_client::GoogleOAuthClient;

#[derive(Debug, Clone)]
pub struct Application {
    pub data_dir: PathBuf,
    pub db_path: PathBuf,
    pub oauth_client: BasicClient,
    pub db: SqlitePool,
    pub google_client: GoogleOAuthClient,
}

impl Application {
    pub async fn setup() -> Result<Self> {
        let data_dir = dirs_next::data_dir()
            .expect("Unable to find data directory")
            .join("so-calendar");
        fs::create_dir_all(&data_dir).expect("Failed to create data directory");

        let db_path = data_dir.join("app.sqlite");

        let oauth_client = configure_oauth_client()?;
        let db = setup_database(&db_path).await?;

        let google_client = GoogleOAuthClient::new(db.clone(), oauth_client.clone());

        Ok(Self {
            data_dir,
            db_path,
            oauth_client,
            db,
            google_client,
        })
    }
}

fn configure_oauth_client() -> Result<BasicClient> {
    let auth_url_raw = String::from("https://accounts.google.com/o/oauth2/v2/auth");
    let auth_url = AuthUrl::new(auth_url_raw).wrap_err("Invalid auth url")?;

    let token_url_raw = String::from("https://www.googleapis.com/oauth2/v3/token");
    let token_url = TokenUrl::new(token_url_raw).wrap_err("Invalid token endpoint")?;

    let client_id = dotenv!("GOOGLE_CLIENT_ID", "Missing Google OAuth Client Id");
    let client_secret = dotenv!("GOOGLE_CLIENT_SECRET", "Missing Google OAuth Client Secret");

    Ok(BasicClient::new(
        ClientId::new(client_id.into()),
        Some(ClientSecret::new(client_secret.into())),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:42069/auth/redirect".into())
            .expect("Invalid redirect uri"),
    ))
}

async fn setup_database(db_path: &PathBuf) -> Result<SqlitePool> {
    let sqlite_options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    let db = SqlitePool::connect_with(sqlite_options)
        .await
        .wrap_err("Failed to connect to sqlite database")?;

    sqlx::migrate!("./migrations").run(&db).await?;

    Ok(db)
}
