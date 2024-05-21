use ratatui::Frame;

use crate::features;

use super::model::CurrentState;
use super::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    match model.current_state {
        CurrentState::Done => {}
        CurrentState::MonthView => features::month_overview::view::render(frame),
        CurrentState::Account => {
            features::account_overview::view::render(frame, &model.application)
        }
        CurrentState::SignUpOptions => {
            features::new_account::view::render(frame, &model.application)
        }
        CurrentState::PendingLogin => todo!(),
    };
}
