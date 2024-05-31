use super::{Account, Calendar};

#[derive(Debug)]
pub struct ManageConnectionsState {
    focused_pane: ManageConnectionPanes,
    pub accounts: Vec<Account>,
    pub calendars: Vec<Calendar>,
    selected_account: Option<Account>,
}

#[derive(Debug)]
enum ManageConnectionPanes {
    Accounts,
    Calendars,
}
