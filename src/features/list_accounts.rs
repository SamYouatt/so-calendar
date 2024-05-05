use std::fmt::Display;

use rusqlite::Connection;

use crate::Application;

#[derive(Debug)]
enum ListAccountErrors {
    FailedToReadAccounts,
}

struct Account {
    email: String,
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email)
    }
}

pub fn handle_list_accounts(application: &Application) {
    let db = Connection::open(&application.db_path).unwrap();

    let mut statement = db
        .prepare("SELECT email FROM accounts")
        .expect("Malformed account select query");

    let accounts: Vec<Account> = statement
        .query_map([], |row| Ok(Account { email: row.get(0)? }))
        .map_err(|_| ListAccountErrors::FailedToReadAccounts)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    for account in accounts {
        println!("{}", account);
    }
}
