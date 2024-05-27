use crate::tui::model::{CurrentState, Model};

pub(crate) mod view;
pub(crate) mod retrieve_accounts;

pub async fn handle_accounts_overview(model: &mut Model) {
    match retrieve_accounts::retrieve_accounts(&model.application).await {
        Ok(accounts) => model.current_state = CurrentState::Account(accounts),
        Err(_) => todo!(),
    };
}
