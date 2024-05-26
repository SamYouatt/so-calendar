use super::model::{CurrentState, Message, Model};

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    // Handle any universal actions
    match msg {
        Message::ManageAccounts => model.current_state = CurrentState::Account,
        Message::Quit => model.current_state = CurrentState::Done,
        _ => {}
    };

    // Handle page specific actions
    match model.current_state {
        CurrentState::Account if msg == Message::New => {
            model.current_state = CurrentState::SignUpOptions
        }
        _ => {}
    }

    None
}
