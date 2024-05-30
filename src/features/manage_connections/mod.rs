use uuid::Uuid;

pub(crate) mod manage_connections_state;
mod retrieve_accounts;
mod retrieve_calendars;

#[derive(Debug)]
pub struct Account {
    id: Uuid,
    pub email: String,
}

#[derive(Debug)]
pub struct Calendar {
    id: Uuid,
    account_id: Uuid,
    title: String,
    description: Option<String>,
}
