use color_eyre::eyre::Result;
use eyre::Context;
use std::fmt::Display;

use crate::Application;

struct Account {
    email: String,
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email)
    }
}

pub fn handle_list_accounts(application: &Application) -> Result<()> {
    let mut statement = application
        .db
        .prepare("SELECT email FROM accounts")
        .expect("Malformed account select query");

    let accounts: Vec<Account> = statement
        .query_map([], |row| Ok(Account { email: row.get(0)? }))
        .wrap_err("Failed to read accounts")?
        .collect::<Result<Vec<_>, _>>()?;

    if accounts.is_empty() {
        println!("No accounts connected...\n");
        println!("To link an account run: `socal account new`");
    }

    for account in accounts {
        println!("{}", account);
    }

    Ok(())
}
