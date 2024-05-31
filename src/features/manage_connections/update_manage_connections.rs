use crate::tui::model::{CurrentState, Model};
use color_eyre::eyre::Result;

use super::{manage_connections_state::{ManageConnectionPanes, ManageConnectionsState}, retrieve_accounts::retrieve_accounts, retrieve_calendars::retrieve_calendars};

pub async fn handle_manage_accounts(model: &mut Model) -> Result<()> {
    let accounts = retrieve_accounts(&model.application.db).await?;
    let calendars = retrieve_calendars(&model.application.db).await?;

    let state = ManageConnectionsState {
        accounts,
        calendars,
        focused_pane: ManageConnectionPanes::Accounts,
        selected_account: None,
    };

    model.current_state = CurrentState::ManageConnections(state);

    Ok(())
}
