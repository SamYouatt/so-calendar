use crate::tui::model::{CurrentState, Message, Model};
use color_eyre::eyre::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
use url::Url;
use thiserror::Error;

#[derive(Debug, Error)]
enum InteractionError {
    #[error("Failed to open browser")]
    FailedOpeningBrowser(#[from] std::io::Error),
    #[error("Failed to copy to system clipboard: {0}")]
    FailedCopyToClipboard(String),
    #[error("Unexpected error during interactive terminal")]
    UnexpectedError(#[from] eyre::Error),
}

pub fn handle_event(model: &mut Model, msg: Message, selected_index: usize) -> Result<()> {
    let auth_url = Url::parse("https://www.google.com").unwrap();

    if msg == Message::Enter {
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

        // tcp handling stuff
        todo!()
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
