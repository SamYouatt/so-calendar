use super::model::{CurrentState, Message, Model};

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::ManageAccounts => model.current_state = CurrentState::Account,
        Message::Quit => model.current_state = CurrentState::Done,
        Message::New => match model.current_state {
            CurrentState::Account => model.current_state = CurrentState::SignUpOptions,
            _ => {}
        },
        Message::LoginStarted => todo!(),
        Message::LoginSuccess => todo!(),
        Message::Back => todo!(),
    };

    None
}
