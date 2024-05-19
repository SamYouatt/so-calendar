use super::MessageSender;

pub struct Model {
    pub running_state: CurrentState,
    pub message_sender: MessageSender,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentState {
    Calendar,
    NewAccountModal,

    // The app should close
    Done,
}

#[derive(Debug)]
pub enum Message {
    OpenNewAccountModal,
    CloseModal,
    Quit,
}
