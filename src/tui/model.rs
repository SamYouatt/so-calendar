use tokio_util::sync::CancellationToken;

use crate::configuration::Application;

use super::MessageSender;

pub struct Model {
    pub application: Application,
    pub current_state: CurrentState,
    pub message_channel: MessageSender,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentState {
    MonthView,

    Account,
    SignUpOptions(usize),
    PendingLogin,

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
