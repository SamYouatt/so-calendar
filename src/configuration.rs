use color_eyre::eyre::Result;
use eyre::Context;
use sqlx::{query, SqlitePool};
use std::{fs, path::PathBuf};

use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

#[derive(Clone)]
pub struct Application {
    pub data_dir: PathBuf,
    pub db_path: PathBuf,
    pub oauth_client: BasicClient,
    pub db: SqlitePool,
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

        Ok(Self {
            data_dir,
            db_path,
            oauth_client,
            db,
        })
    }
}

fn configure_oauth_client() -> Result<BasicClient> {
    let auth_url_raw = String::from("https://accounts.google.com/o/oauth2/v2/auth");
    let auth_url = AuthUrl::new(auth_url_raw).wrap_err("Invalid auth url")?;

    let token_url_raw = String::from("https://www.googleapis.com/oauth2/v3/token");
    let token_url = TokenUrl::new(token_url_raw).wrap_err("Invalid token endpoint")?;

    Ok(BasicClient::new(
        ClientId::new(
            "357015344564-7rf7b47n7add82k2t3hajfhq2pklthen.apps.googleusercontent.com".into(),
        ),
        Some(ClientSecret::new(
            "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".into(),
        )),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:42069/auth/redirect".into())
            .expect("Invalid redirect uri"),
    ))
}

async fn setup_database(db_path: &PathBuf) -> Result<SqlitePool> {
    let db = SqlitePool::connect(db_path.to_str().expect("No db path found"))
        .await
        .wrap_err("Failed to connect to sqlite database")?;

    query!(
        "CREATE TABLE IF NOT EXISTS accounts(
                    id integer PRIMARY KEY,
                    email text NOT NULL UNIQUE,
                    access_token text NOT NULL,
                    refresh_token text NOT NULL,
                    expires_at text NOT NULL
                )"
    )
    .execute(&db)
    .await
    .wrap_err("Failed to set up accounts table")?;

    Ok(db)
}
