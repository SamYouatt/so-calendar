use crate::tui::model::{CurrentState, Message, Model};
use color_eyre::eyre::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use super::handle_new_account::account_signin_task;

#[derive(Debug, Error)]
enum InteractionError {
    #[error("Failed to open browser")]
    FailedOpeningBrowser(#[from] std::io::Error),
    #[error("Failed to copy to system clipboard: {0}")]
    FailedCopyToClipboard(String),
    #[error("Unexpected error during interactive terminal")]
    UnexpectedError(#[from] eyre::Error),
}

pub fn handle_list_interaction(
    model: &mut Model,
    msg: Message,
    selected_index: usize,
) -> Result<()> {
    if matches!(msg, Message::Enter) {
        item_selected(selected_index, model)?;
    };

    let selection = match msg {
        Message::Up => {
            if selected_index == 0 {
                1
            } else {
                0
            }
        }
        Message::Down => {
            if selected_index == 1 {
                0
            } else {
                1
            }
        }
        _ => selected_index,
    };

    model.current_state = CurrentState::SignUpOptions(selection);

    Ok(())
}

fn item_selected(selected_index: usize, model: &Model) -> Result<()> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, _) = model
        .application
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".into()))
        .add_scope(Scope::new("email".into()))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar".into(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    match selected_index {
        0 => open::that(auth_url.as_str())?,
        _ => {
            let mut clipboard = ClipboardContext::new()
                .map_err(|e| InteractionError::FailedCopyToClipboard(e.to_string()))?;
            clipboard
                .set_contents(auth_url.into())
                .map_err(|e| InteractionError::FailedCopyToClipboard(e.to_string()))?;
        }
    }

    let application = model.application.clone();

    let cancellation_token = CancellationToken::new();
    model.message_channel
        .send(Message::LoginStarted(cancellation_token.clone()))
        .expect("Message channel should not be closed");


    let message_channel = model.message_channel.clone();
    tokio::spawn(
        async move { account_signin_task(application, message_channel, pkce_verifier, cancellation_token).await },
    );

    Ok(())
}
