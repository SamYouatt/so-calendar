use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use chrono::{DateTime, Utc};
use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    RedirectUrl, TokenResponse, TokenUrl,
};
use rusqlite::Connection;
use url::Url;

use crate::Application;

struct Account {
    access_token: String,
    refresh_token: String,
    email: String,
    expiry: DateTime<Utc>,
}

pub fn handle_new_account(application: &Application) {
    // Create the redirect url to show to the user
    let auth_url_raw = String::from("https://accounts.google.com/o/oauth2/v2/auth");
    let auth_url = AuthUrl::new(auth_url_raw).expect("Invalid auth endpoint");

    let token_url_raw = String::from("https://www.googleapis.com/oauth2/v3/token");
    let token_url = TokenUrl::new(token_url_raw).expect("Invalid token endpoint");

    let redirect_url = "http://localhost:42069/login";

    let client = BasicClient::new(
        ClientId::new(
            "357015344564-7rf7b47n7add82k2t3hajfhq2pklthen.apps.googleusercontent.com".into(),
        ),
        Some(ClientSecret::new(
            "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".into(),
        )),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.into()).unwrap());

    let url = format!(
        "{}?response_type=code&client_id={}&scope=openid%20email%20https://www.googleapis.com/auth/calendar&redirect_uri={}",
        "https://accounts.google.com/o/oauth2/v2/auth",
        client.client_id().as_str(),
        client
            .redirect_url()
            .expect("Couldn't find open id client redirect")
            .as_str()
    );

    println!("Open the link below in your browser to connect a Google account");
    println!("> {}", url);

    // Create a tcp server to listen for the redirect response
    let address = "localhost:42069";
    let listener = TcpListener::bind(&address).expect("Failed to bind tcp listener");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let result = handle_connection(stream, &address, &client, &application);
        println!("Result: {:?}", result);
    }
}

fn handle_connection(
    mut stream: TcpStream,
    address: &str,
    oauth_client: &BasicClient,
    application: &Application,
) -> Result<(), NewAccountError> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let pieces: Vec<_> = request_line.split_whitespace().collect();

    if pieces.len() != 3 || pieces[0] != "GET" || pieces[2] != "HTTP/1.1" {
        return Err(NewAccountError::InvalidRedirectResponse);
    }

    let absolute_url = format!("{}/{}", address, pieces[1]);
    let redirect_request_url =
        Url::parse(&absolute_url).map_err(|_| NewAccountError::InvalidRedirectUrl)?;

    let query_pairs: HashMap<_, _> = redirect_request_url.query_pairs().collect();

    let auth_code = query_pairs
        .get("code")
        .map(|code| AuthorizationCode::new(code.to_string()))
        .ok_or(NewAccountError::MissingAuthCode)?;

    let auth_token = oauth_client
        .exchange_code(auth_code)
        .request(http_client)
        .map_err(|e| {
            println!("Error: {:?}", e);
            NewAccountError::FailedTokenExchange
        })?;

    let account = Account {
        access_token: auth_token.access_token().secret().into(),
        refresh_token: auth_token
            .refresh_token()
            .ok_or(NewAccountError::FailedTokenExchange)?
            .secret()
            .into(),
        email: "test@test.com".into(),
        expiry: Utc::now()
            + auth_token
                .expires_in()
                .ok_or(NewAccountError::FailedTokenExchange)?,
    };

    store_account(account, application)?;

    return Ok(());
}

fn store_account(account: Account, application: &Application) -> Result<(), NewAccountError> {
    let db = Connection::open(&application.db_path).unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS accounts(
                id integer PRIMARY KEY,
                email text NOT NULL UNIQUE,
                access_token text NOT NULL,
                refresh_token text NOT NULL,
                expires_at text NOT NULL
            )",
        [],
    )
    .map_err(|e| {
        println!("Error creating accounts table: {:?}", e);
        NewAccountError::SqliteError
    })?;

    db.execute(
        "INSERT INTO accounts (email, access_token, refresh_token, expires_at) VALUES (?1, ?2, ?3, ?4)",
        [account.email, account.access_token, account.refresh_token, account.expiry.to_rfc3339()],
        )
        .map_err(|e| {
            println!("Error inserting account: {:?}", e);
            NewAccountError::SqliteError
        })?;

    Ok(())
}

#[derive(Debug)]
enum NewAccountError {
    InvalidRedirectResponse,
    InvalidRedirectUrl,
    MissingAuthCode,
    FailedTokenExchange,
    SqliteError,
}
