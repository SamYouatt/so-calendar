use thiserror::Error;

pub(crate) mod handle_new_account;
pub(crate) mod store_account;
pub(crate) mod tcp_request_handler;
pub(crate) mod view;
pub(crate) mod handle_event;

#[derive(Debug, Error)]
enum InteractionError {
    #[error("Failed to open browser")]
    FailedOpeningBrowser(#[from] std::io::Error),
    #[error("Failed to copy to system clipboard: {0}")]
    FailedCopyToClipboard(String),
    #[error("Unexpected error during interactive terminal")]
    UnexpectedError(#[from] eyre::Error),
}
