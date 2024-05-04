use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    RedirectUrl, TokenUrl,
};
use url::Url;

pub fn handle_new_account() {
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

        let result = handle_connection(stream, &address, &client);
        println!("Result: {:?}", result);
    }
}

fn handle_connection(
    mut stream: TcpStream,
    address: &str,
    oauth_client: &BasicClient,
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
        });

    return Ok(());
}

#[derive(Debug)]
enum NewAccountError {
    InvalidRedirectResponse,
    InvalidRedirectUrl,
    MissingAuthCode,
    FailedTokenExchange,
}
