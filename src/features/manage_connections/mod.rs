use uuid::Uuid;

pub(crate) mod manage_connections_state;
pub(crate) mod manage_connections_view;
pub(crate) mod update_manage_connections;
mod retrieve_accounts;
mod retrieve_calendars;

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
