use super::model::{CurrentState, Message, Model};

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::OpenNewAccountModal => {}
        Message::CloseModal => todo!(),
        Message::Quit => model.running_state = CurrentState::Done,
    };

    None
}
