use ratatui::Frame;

use crate::features;

use super::model::CurrentState;
use super::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    match &model.current_state {
        CurrentState::Done => {}

        CurrentState::MonthView => features::month_view::view::render(frame, &model.events_state),
        CurrentState::DaysView(_) => features::days_view::view::render(frame),

        CurrentState::ManageConnections(page_state) => {
            features::manage_connections::manage_connections_view::render(page_state, frame)
        }

        CurrentState::SignUpOptions(selected_index) => {
            features::new_account::view::render_sign_in_options(frame, *selected_index)
        }
        CurrentState::PendingLogin(_) => {
            features::new_account::view::render_waiting_for_signin(frame)
        }
    };
}
