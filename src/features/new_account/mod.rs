pub(crate) mod handle_new_account;
pub(crate) mod tcp_request_handler;
pub(crate) mod store_account;

#[derive(Debug)]
pub enum NewAccountError {
    InvalidRedirectResponse,
    InvalidRedirectUrl,
    MissingAuthCode,
    FailedTokenExchange,
    FailedProfileRetrieve,
    SqliteError,
}
