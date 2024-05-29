use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
    time::Duration,
};

use chrono::Utc;
use color_eyre::eyre::Result;
use eyre::eyre;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier,
    TokenResponse,
};
use url::Url;

use crate::configuration::Application;

use super::{
    account_signin_task::{Account, UserProfile},
    store_account::store_account,
};

pub async fn handle_tcp_request(
    mut stream: TcpStream,
    address: &str,
    oauth_client: &BasicClient,
    application: &Application,
    pkce_verifier: PkceCodeVerifier,
) -> Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .ok_or(eyre!("Badly formed tcp request"))??;
    let pieces: Vec<_> = request_line.split_whitespace().collect();

    if pieces.len() != 3 || pieces[0] != "GET" || pieces[2] != "HTTP/1.1" {
        return Err(eyre!("Badly formed tcp request: {pieces:?}"));
    }

    let absolute_url = format!("{}/{}", address, pieces[1]);
    let redirect_request_url = Url::parse(&absolute_url)?;

    let query_pairs: HashMap<_, _> = redirect_request_url.query_pairs().collect();

    let auth_code = query_pairs
        .get("code")
        .map(|code| AuthorizationCode::new(code.to_string()))
        .ok_or(eyre!(
            "Missing auth code from auth request: {redirect_request_url:?}"
        ))?;

    let auth_token = oauth_client
        .exchange_code(auth_code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await?;

    let access_token = auth_token.access_token().secret().to_string();

    let client = reqwest::Client::new();
    let profile = client
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(&access_token)
        .send()
        .await?;

    let profile = profile.json::<UserProfile>().await?;

    let account = Account {
        access_token,
        refresh_token: auth_token
            .refresh_token()
            .ok_or(eyre!("Expected refresh token but none returned"))?
            .secret()
            .into(),
        email: profile.email,
        expiry: Utc::now() + auth_token.expires_in().unwrap_or(Duration::from_secs(3600)),
    };

    store_account(account, application).await?;

    Ok(())
}
