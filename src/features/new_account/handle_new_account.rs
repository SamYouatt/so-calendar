use std::net::TcpListener;

use chrono::{DateTime, Utc};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use serde::Deserialize;

use crate::{features::new_account::tcp_request_handler::handle_tcp_request, Application};

pub struct Account {
    pub access_token: String,
    pub refresh_token: String,
    pub email: String,
    pub expiry: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UserProfile {
    pub email: String,
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
