use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use chrono::{DateTime, Utc};
use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthorizationCode, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, Scope, TokenResponse,
};
use rusqlite::Connection;
use serde::Deserialize;
use url::Url;

use crate::Application;

struct Account {
    access_token: String,
    refresh_token: String,
    email: String,
    expiry: DateTime<Utc>,
}

#[derive(Deserialize)]
struct UserProfile {
    email: String,
}

pub fn handle_new_account(application: &Application) {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, _) = application
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".into()))
        .add_scope(Scope::new("email".into()))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar".into(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Open the link below in your browser to connect a Google account");
    println!("> {}", auth_url);

    // Create a tcp server to listen for the redirect response
    let address = "localhost:42069";
    let listener = TcpListener::bind(&address).expect("Failed to bind tcp listener");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // TODO: handle errors and exit
        let _ = handle_tcp_request(
            stream,
            &address,
            &application.oauth_client,
            &application,
            pkce_verifier,
        );
        return;
    }
}

fn handle_tcp_request(
    mut stream: TcpStream,
    address: &str,
    oauth_client: &BasicClient,
    application: &Application,
    pkce_verifier: PkceCodeVerifier,
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
        .set_pkce_verifier(pkce_verifier)
        .request(http_client)
        .map_err(|e| {
            println!("Error: {:?}", e);
            NewAccountError::FailedTokenExchange
        })?;

    let access_token = auth_token.access_token().secret().to_string();

    let client = reqwest::blocking::Client::new();
    let profile = client
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(&access_token)
        .send()
        .map_err(|e| {
            println!("Failed to retrieve profile: {:?}", e);
            NewAccountError::FailedProfileRetrieve
        })?;

    let profile = profile.json::<UserProfile>().map_err(|e| {
        println!("Failed to deserialize profile response: {:?}", e);
        NewAccountError::FailedProfileRetrieve
    })?;

    let account = Account {
        access_token,
        refresh_token: auth_token
            .refresh_token()
            .ok_or(NewAccountError::FailedTokenExchange)?
            .secret()
            .into(),
        email: profile.email,
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
    FailedProfileRetrieve,
    SqliteError,
}
