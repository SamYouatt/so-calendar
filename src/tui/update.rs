use crate::features;
use color_eyre::eyre::Result;

use super::model::{CurrentState, Message, Model};

pub fn update(model: &mut Model, msg: Message) -> Result<Option<Message>> {
    // Handle any universal actions
    match msg {
        Message::Quit => model.current_state = CurrentState::Done,
        Message::ManageAccounts => model.current_state = CurrentState::Account,
        Message::LoginStarted => model.current_state = CurrentState::PendingLogin,
        Message::LoginSuccess => model.current_state = CurrentState::Account,
        _ => {}
    };

    // Handle page specific actions
    match model.current_state {
        CurrentState::Account if msg == Message::New => {
            model.current_state = CurrentState::SignUpOptions(0)
        }
        CurrentState::SignUpOptions(list_state) => {
            features::new_account::handle_event::handle_list_interaction(model, msg, list_state)?
        }
        _ => {}
    }

    Ok(None)
}
