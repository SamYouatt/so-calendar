use crate::tui::model::{CurrentState, Model};
use color_eyre::eyre::Result;

use super::{
    manage_connections_state::{ManageConnectionPanes, ManageConnectionsState},
    retrieve_accounts::retrieve_accounts,
    retrieve_calendars::retrieve_calendars,
};

pub async fn handle_manage_accounts(model: &mut Model) -> Result<()> {
    let accounts = retrieve_accounts(&model.application.db).await?;
    let calendars = retrieve_calendars(&model.application.db).await?;

    let state = ManageConnectionsState {
        accounts,
        calendars,
        focused_pane: ManageConnectionPanes::Accounts,
        selected_account_index: 0,
        selected_calendar_index: None,
    };

    model.current_state = CurrentState::ManageConnections(state);

    Ok(())
}

pub fn handle_up_message(model: &mut Model) {
    let CurrentState::ManageConnections(ref mut internal_state) = model.current_state else {
        return;
    };

    if internal_state.focused_pane == ManageConnectionPanes::Accounts {
        let current_index = internal_state.selected_account_index;
        let max_index = internal_state.accounts.len() - 1;

        if current_index == 0 {
            internal_state.selected_account_index = max_index;
        } else {
            internal_state.selected_account_index = current_index - 1;
        }
    }
}

pub fn handle_down_message(model: &mut Model) {
    let CurrentState::ManageConnections(ref mut internal_state) = model.current_state else {
        return;
    };

    if internal_state.focused_pane == ManageConnectionPanes::Accounts {
        let current_index = internal_state.selected_account_index;
        let max_index = internal_state.accounts.len() - 1;

        if current_index == max_index {
            internal_state.selected_account_index = 0;
        } else {
            internal_state.selected_account_index = current_index + 1;
        }
    }
}
