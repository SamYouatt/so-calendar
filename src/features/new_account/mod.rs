use thiserror::Error;

mod account_signin_task;
pub(crate) mod handle_event;
mod populate_new_calendars;
mod store_account;
mod tcp_request_handler;
pub(crate) mod view;

#[derive(Debug, Error)]
enum InteractionError {
    #[error("Failed to open browser")]
    FailedOpeningBrowser(#[from] std::io::Error),
    #[error("Failed to copy to system clipboard: {0}")]
    FailedCopyToClipboard(String),
    #[error("Unexpected error during interactive terminal")]
    UnexpectedError(#[from] eyre::Error),
}
