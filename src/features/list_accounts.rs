use color_eyre::eyre::Result;
use eyre::Context;
use serde::Deserialize;
use std::fmt::Display;

use super::oauth_http_client::GoogleOAuthClient;
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
    println!("-------------");
    super_secret_test(&application);
    println!("-------------");

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

#[derive(Deserialize, Debug)]
struct CalendarListResponse {
    items: Vec<CalendarList>
}

#[derive(Deserialize, Debug)]
struct CalendarList {
    description: Option<String>,
    id: String,
    summary: String,
}

fn super_secret_test(application: &Application) {
    let oauth_client = GoogleOAuthClient::new(&application.db, &application.oauth_client);
    let client = reqwest::blocking::Client::new();
    let request = client.get("www.google.com");
    let _test = oauth_client.send("sdyouatt@gmail.com".into(), request);
    let test = client
        .get("https://www.googleapis.com/calendar/v3/users/me/calendarList");
    let test = oauth_client.send("sdyouatt@gmail.com".to_string(), test).unwrap();

    println!("received: {:#?}", test);

    // println!("bytes: {:?}", test.bytes()?);

    let json: CalendarListResponse = test.json().unwrap();

    println!("Deserialized: {:?}", json);
}
