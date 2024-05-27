use tokio_util::sync::CancellationToken;

use crate::{configuration::Application, features::account_overview::retrieve_accounts::Account};

use super::MessageSender;

pub struct Model {
    pub application: Application,
    pub current_state: CurrentState,
    pub message_channel: MessageSender,
}

#[derive(Debug)]
pub enum CurrentState {
    DaysView,
    MonthView,

    Account(Vec<Account>),
    SignUpOptions(usize),
    PendingLogin(CancellationToken),

    Done,
}

#[derive(Debug)]
pub enum Message {
    ManageAccounts,
    LoginStarted(CancellationToken),
    LoginSuccess,

    Down,
    Up,
    Enter,

    New,
    Back,
    Quit,
}
