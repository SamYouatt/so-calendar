use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use chrono::Utc;
use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthorizationCode, PkceCodeVerifier, TokenResponse,
};
use url::Url;

use crate::configuration::Application;

use super::{
    handle_new_account::{Account, UserProfile},
    store_account::store_account,
    NewAccountError,
};

pub fn handle_tcp_request(
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
