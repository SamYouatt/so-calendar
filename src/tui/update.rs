use crate::features;
use color_eyre::eyre::Result;

use super::model::{CurrentState, Message, Model};

pub fn update(model: &mut Model, msg: Message) -> Result<Option<Message>> {
    // Handle any unique actions
    match msg {
        Message::Quit => graceful_shutdown(model),

        Message::ManageAccounts => model.current_state = CurrentState::Account,
        Message::LoginStarted(ref cancellation_token) => {
            model.current_state = CurrentState::PendingLogin(cancellation_token.clone())
        }
        Message::LoginSuccess => model.current_state = CurrentState::Account,

        _ => {}
    };

    // Handle non-unique actions e.g. up/down/select
    match model.current_state {
        CurrentState::Account if matches!(msg, Message::New) => {
            model.current_state = CurrentState::SignUpOptions(0)
        }
        CurrentState::SignUpOptions(list_state) => {
            features::new_account::handle_event::handle_list_interaction(model, msg, list_state)?
        }
        _ => {}
    }

    Ok(None)
}

// Graceful shutdown of any remaining tasks
fn graceful_shutdown(model: &mut Model) {
    match &model.current_state {
        CurrentState::PendingLogin(cancellation_token) => cancellation_token.cancel(),
        _ => {}
    };

    model.current_state = CurrentState::Done;
}
