use std::{io, net::TcpListener, thread, time::Duration};

use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use oauth2::PkceCodeVerifier;
use serde::Deserialize;
use tokio_util::sync::CancellationToken;

use crate::{
    features::new_account::tcp_request_handler::handle_tcp_request,
    tui::{model::Message, MessageSender},
    Application,
};

use super::populate_new_calendars::populate_new_calendars;

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

pub async fn account_signin_task(
    application: Application,
    message_channel: MessageSender,
    pkce_verifier: PkceCodeVerifier,
    cancellation_token: CancellationToken,
) -> Result<()> {
    let address = "localhost:42069";
    let listener = TcpListener::bind(address).expect("Failed to bind tcp listener");
    listener
        .set_nonblocking(true)
        .expect("Unable to set listener to non-blocking");

    loop {
        if cancellation_token.is_cancelled() {
            return Ok(());
        }

        match listener.accept() {
            Ok((stream, _)) => {
                let email = handle_tcp_request(
                    stream,
                    address,
                    &application.oauth_client,
                    &application,
                    pkce_verifier,
                )
                .await?;

                populate_new_calendars(email, &application).await?;

                message_channel
                    .send(Message::LoginSuccess)
                    .expect("Message channel should not be closed");

                break;
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
