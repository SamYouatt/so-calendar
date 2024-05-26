use crate::configuration::Application;

use super::{task_manager::TaskManager, MessageSender};

pub struct Model<'a> {
    pub application: Application,
    pub current_state: CurrentState,
    pub message_channel: MessageSender,
    pub task_manager: &'a mut TaskManager,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentState {
    MonthView,

    Account,
    SignUpOptions(usize),
    PendingLogin,

    Done,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    ManageAccounts,
    LoginStarted,
    LoginSuccess,

    Down,
    Up,
    Enter,

    New,
    Back,
    Quit,
}
