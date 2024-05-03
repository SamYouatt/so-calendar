use oauth2::{basic::BasicClient, AuthUrl, ClientId, RedirectUrl, TokenUrl};

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
        None,
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
}
