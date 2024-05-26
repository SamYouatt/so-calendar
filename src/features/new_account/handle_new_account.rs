use std::net::TcpListener;

use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use eyre::Context;
use oauth2::{CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope};
use serde::Deserialize;
use thiserror::Error;
use url::Url;

use crate::{
    features::new_account::tcp_request_handler::handle_tcp_request,
    tui::{model::Message, MessageSender},
    Application,
};

pub struct Account {
    pub access_token: String,
    pub refresh_token: String,
    pub email: String,
    pub expiry: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UserProfile {
    pub email: String,
}

pub async fn handle_new_account(
    application: &Application,
    message_channel: MessageSender,
    pkce_verifier: PkceCodeVerifier,
) -> Result<()> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, _) = application
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".into()))
        .add_scope(Scope::new("email".into()))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar".into(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let address = "localhost:42069";
    let listener = TcpListener::bind(address).expect("Failed to bind tcp listener");

    for stream in listener.incoming() {
        let stream = stream.wrap_err("Error accepting tcp connection")?;

        // TODO: handle errors and exit
        handle_tcp_request(
            stream,
            address,
            &application.oauth_client,
            application,
            pkce_verifier,
        )
        .await?;

        message_channel
            .send(Message::LoginSuccess)
            .expect("Message channel should not be closed");

        break;
    }

    Ok(())
}

#[derive(Debug, Error)]
enum InteractionError {
    #[error("Failed to open browser")]
    FailedOpeningBrowser(#[from] std::io::Error),
    #[error("Failed to copy to system clipboard: {0}")]
    FailedCopyToClipboard(String),
    #[error("Unexpected error during interactive terminal")]
    UnexpectedError(#[from] eyre::Error),
}
