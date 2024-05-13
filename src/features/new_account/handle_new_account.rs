use eyre::eyre;
use std::net::TcpListener;

use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use eyre::Context;
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use serde::Deserialize;
use thiserror::Error;
use url::Url;

use crate::{features::new_account::tcp_request_handler::handle_tcp_request, Application};

use super::url_tui::{init_terminal, restore_terminal, update, view, Message, Model, RunningState};

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

pub fn handle_new_account(application: &Application) -> Result<()> {
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

    // TODO: handle failure to open browser or clipboard by presenting plain link
    interactive_auth_prompt(auth_url)?;

    println!("Waiting for you to log in...");

    let address = "localhost:42069";
    let listener = TcpListener::bind(&address).expect("Failed to bind tcp listener");

    for stream in listener.incoming() {
        let stream = stream.wrap_err("Error accepting tcp connection")?;

        // TODO: handle errors and exit
        let _ = handle_tcp_request(
            stream,
            &address,
            &application.oauth_client,
            &application,
            pkce_verifier,
        );

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

fn interactive_auth_prompt(auth_url: Url) -> Result<(), InteractionError> {
    let mut terminal = init_terminal()?;

    let mut model = Model::new();

    while model.state == RunningState::Running {
        terminal.draw(|frame| view(&mut model, frame))?;

        if let Some(message) = handle_event().wrap_err("Unable to read terminal input")? {
            update(&mut model, message);
        }
    }

    match model.state {
        RunningState::SelectionMade(selection) => {
            restore_terminal(&mut terminal)?;

            match selection {
                super::url_tui::LoginOption::OpenBrowser => {
                    open::that(auth_url.as_str())?;
                }
                super::url_tui::LoginOption::CopyToClipboard => {
                    let mut clipboard = ClipboardContext::new()
                        .map_err(|e| InteractionError::FailedCopyToClipboard(e.to_string()))?;
                    clipboard
                        .set_contents(auth_url.into())
                        .map_err(|e| InteractionError::FailedCopyToClipboard(e.to_string()))?;
                }
            };

            Ok(())
        }
        RunningState::Exited => {
            restore_terminal(&mut terminal)?;
            std::process::exit(0);
        }
        RunningState::Running => unreachable!(),
    }
}

fn handle_event() -> Result<Option<Message>> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            return Ok(handle_key(key));
        }
    }

    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Up => Some(Message::Previous),
        KeyCode::Down => Some(Message::Next),
        KeyCode::Enter => Some(Message::Select),
        KeyCode::Esc => Some(Message::Quit),
        _ => None,
    }
}
