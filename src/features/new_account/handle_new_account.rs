use std::net::TcpListener;

use chrono::{DateTime, Utc};
use copypasta::{ClipboardContext, ClipboardProvider};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use serde::Deserialize;
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

pub fn handle_new_account(application: &Application) {
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

    interactive_auth_prompt(auth_url);

    println!("Waiting for you to log in...");

    let address = "localhost:42069";
    let listener = TcpListener::bind(&address).expect("Failed to bind tcp listener");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // TODO: handle errors and exit
        let _ = handle_tcp_request(
            stream,
            &address,
            &application.oauth_client,
            &application,
            pkce_verifier,
        );
        return;
    }
}

fn interactive_auth_prompt(auth_url: Url) {
    let mut terminal = init_terminal().unwrap();

    let mut model = Model::new();

    while model.state == RunningState::Running {
        terminal.draw(|frame| view(&mut model, frame)).unwrap();

        let message = handle_event();

        if message.is_some() {
            update(&mut model, message.unwrap());
        }
    }

    match model.state {
        RunningState::SelectionMade(selection) => {
            restore_terminal(&mut terminal).unwrap();

            // TODO: either copy to clipboard or open the browser
            // clear the terminal and inform the user that we are waiting for them to login
            // then return control back to the caller to handle the tcp request

            match selection {
                super::url_tui::LoginOption::OpenBrowser => {
                    open::that(auth_url.as_str()).unwrap();
                },
                super::url_tui::LoginOption::CopyToClipboard => {
                    let mut clipboard = ClipboardContext::new().unwrap();
                    clipboard.set_contents(auth_url.into()).unwrap();
                }
            };

            return;
        }
        RunningState::Exited => std::process::exit(0),
        RunningState::Running => unreachable!(),
    }
}

fn handle_event() -> Option<Message> {
    if let Event::Key(key) = event::read().unwrap() {
        if key.kind == KeyEventKind::Press {
            return handle_key(key);
        }
    }

    None
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
