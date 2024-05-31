use super::{Account, Calendar};

#[derive(Debug)]
pub struct ManageConnectionsState {
    pub focused_pane: ManageConnectionPanes,
    pub accounts: Vec<Account>,
    pub calendars: Vec<Calendar>,
    pub selected_account: Option<Account>,
}

#[derive(Debug)]
pub enum ManageConnectionPanes {
    Accounts,
    Calendars,
}
