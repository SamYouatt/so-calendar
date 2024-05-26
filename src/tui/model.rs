use crate::configuration::Application;

pub struct Model {
    pub application: Application,
    pub current_state: CurrentState,
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

    New,
    Back,
    Quit,
}
