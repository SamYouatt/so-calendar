use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use tokio_util::sync::CancellationToken;

use super::{model::{Message, Model}, MessageSender};

pub fn handle_event(
    _model: &Model,
    message_sender: MessageSender,
    cancellation_token: CancellationToken,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let tick_rate = Duration::from_millis(5);

        loop {
            if cancellation_token.is_cancelled() {
                break;
            }

            let message: Option<Message> = match event::poll(tick_rate) {
                Ok(true) => match event::read() {
                    Ok(Event::Key(key)) if key.kind == event::KeyEventKind::Press => {
                        handle_key(key)
                    }
                    Ok(_) => None,
                    Err(_) => None,
                },
                Ok(false) => None,
                Err(_) => None,
            };

            if message.is_none() {
                continue;
            }

            if let Err(_) = message_sender.send(message.unwrap()) {
                // TODO: nicer handling here
                break;
            }
        }
    })
}

pub fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('a') => Some(Message::OpenAccountView),
        _ => None,
    }
}