use crate::configuration::Application;

use super::MessageSender;

pub struct Model {
    pub application: Application,
    pub current_state: CurrentState,
    pub message_sender: MessageSender,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentState {
    MonthView,
    AccountView,
    // The app should close
    Done,
}

#[derive(Debug)]
pub enum Message {
    OpenAccountView,
    Quit,
}
