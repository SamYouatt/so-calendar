use super::{Account, Calendar};

#[derive(Debug)]
pub struct ManageConnectionsState {
    pub focused_pane: ManageConnectionPanes,
    pub accounts: Vec<Account>,
    pub calendars: Vec<Calendar>,
    pub selected_account_index: usize,
    pub selected_calendar_index: Option<usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ManageConnectionPanes {
    Accounts,
    Calendars,
}
