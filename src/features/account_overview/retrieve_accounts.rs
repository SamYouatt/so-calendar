use color_eyre::eyre::Result;
use eyre::Context;
use std::fmt::Display;

use crate::configuration::Application;

pub struct Account {
    pub email: String,
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email)
    }
}

pub fn retrieve_accounts(application: &Application) -> Result<Vec<Account>> {
    let mut statement = application
        .db
        .prepare("SELECT email FROM accounts")
        .expect("Malformed account select query");

    let accounts: Vec<Account> = statement
        .query_map([], |row| Ok(Account { email: row.get(0)? }))
        .wrap_err("Failed to read accounts")?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(accounts)
}
