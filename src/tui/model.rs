use tokio_util::sync::CancellationToken;

use crate::{
    configuration::Application,
    features::{
        days_view::days_view_state::DaysViewState,
        manage_connections::manage_connections_state::ManageConnectionsState,
    },
};

use super::MessageSender;

pub struct Model {
    pub application: Application,
    pub current_state: CurrentState,
    pub message_channel: MessageSender,
}

#[derive(Debug)]
pub enum CurrentState {
    DaysView(DaysViewState),
    MonthView,

    ManageConnections(ManageConnectionsState),
    SignUpOptions(usize),
    PendingLogin(CancellationToken),

    Done,
}

#[derive(Debug)]
pub enum Message {
    DaysView,

    ManageAccounts,
    LoginStarted(CancellationToken),
    LoginSuccess,

    ManageCalendars,

    Down,
    Up,
    Enter,

    New,
    Back,
    Quit,
}
