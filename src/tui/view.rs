use ratatui::Frame;

use crate::features;

use super::model::CurrentState;
use super::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    match &model.current_state {
        CurrentState::Done => {}
        CurrentState::MonthView => features::month_overview::view::render(frame),
        CurrentState::DaysView => features::days_view::view::render(frame),
        CurrentState::Account(accounts) => {
            features::account_overview::view::render(accounts, frame, &model.application)
        }
        CurrentState::SignUpOptions(selected_index) => {
            features::new_account::view::render_sign_in_options(
                frame,
                &model.application,
                *selected_index,
            )
        }
        CurrentState::PendingLogin(_) => {
            features::new_account::view::render_waiting_for_signin(frame, &model.application)
        }
    };
}
