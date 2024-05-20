use super::model::{CurrentState, Message, Model};

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::OpenAccountView => model.running_state = CurrentState::AccountView,
        Message::Quit => model.running_state = CurrentState::Done,
    };

    None
}
