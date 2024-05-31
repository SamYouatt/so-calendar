use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use std::{fs::create_dir_all, path::PathBuf};

use socal::{configuration::Application, features::oauth_http_client::GoogleOAuthClient, tui::model::Model};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tokio::sync::mpsc;
use uuid::Uuid;

mod manage_connections;

pub async fn create_default_model() -> Model {
    // random test id so tests can be isolated
    let test_uuid = Uuid::new_v4();

    let data_dir = format!("/tmp/{}", test_uuid.to_string());
    let data_dir = PathBuf::from(data_dir);

    create_dir_all(&data_dir).expect("failed to create temporary test folder");

    let db_path = data_dir.join("app.sqlite");

    let db = setup_database(&db_path).await;

    let oauth_client = configure_oauth_client();
    let google_client = GoogleOAuthClient::new(db.clone(), oauth_client.clone());

    let application = Application {
        data_dir: data_dir.into(),
        db,
        db_path,
        oauth_client,
        google_client,
    };

    let (message_sender, _message_receiver) = mpsc::unbounded_channel();

    Model {
        application,
        current_state: socal::tui::model::CurrentState::DaysView,
        message_channel: message_sender,
    }
}

fn configure_oauth_client() -> BasicClient {
    let auth_url_raw = String::from("https://accounts.google.com/o/oauth2/v2/auth");
    let auth_url = AuthUrl::new(auth_url_raw).expect("Invalid auth url");

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
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:42069/auth/redirect".into())
            .expect("Invalid redirect uri"),
    )
}

async fn setup_database(db_path: &PathBuf) -> SqlitePool {
    let sqlite_options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    println!("{:?}", db_path);

    let db = SqlitePool::connect_with(sqlite_options)
        .await
        .expect("Failed to connect to sqlite database");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to migrate test db");

    db
}
