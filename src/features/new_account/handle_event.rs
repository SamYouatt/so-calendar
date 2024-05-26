use crate::tui::model::{CurrentState, Message, Model};

pub fn handle_event(model: &mut Model, msg: Message, selected_index: usize) {
    let selection = match msg {
        Message::Up => {
            if selected_index == 0 {
                1
            } else {
                0
            }
        }
        Message::Down => {
            if selected_index == 1 {
                0
            } else {
                1
            }
        }
        _ => selected_index,
    };

    model.current_state = CurrentState::SignUpOptions(selection);
}
