pub mod manage_connections_state;
pub(crate) mod manage_connections_view;
mod retrieve_accounts;
mod retrieve_calendars;
pub(crate) mod update_manage_connections;

#[derive(Debug)]
pub struct Account {
    id: i64,
    pub email: String,
}

#[derive(Debug)]
pub struct Calendar {
    id: i64,
    account_id: i64,
    title: String,
    description: Option<String>,
}
