use ratatui::Frame;

use crate::features;

use super::model::CurrentState;
use super::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    match model.running_state {
        CurrentState::MonthView => features::month_overview::view::render(frame),
        CurrentState::AccountView => features::account_overview::view::render(frame),
        CurrentState::Done => {}
    };

}
