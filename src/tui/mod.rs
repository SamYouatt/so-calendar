use std::io::stdout;

use chrono::{DateTime, Duration, Local, NaiveDate, NaiveTime, Utc};
use color_eyre::eyre::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio_util::sync::CancellationToken;

use crate::{
    configuration::Application,
    domain::events::{DayEvent, Event}, features::fetch_events::fetch_events::run_fetch_events_task,
};

use self::model::{CurrentState, Message, Model};
use self::update::update;
use self::view::view;
use self::{handle_event::handle_event, model::EventsState};

pub(crate) mod handle_event;
pub mod model;
pub mod update;
pub(crate) mod util;
pub(crate) mod view;

pub type MessageSender = mpsc::UnboundedSender<Message>;

pub fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

async fn main_loop(
    terminal: &mut Terminal<impl Backend>,
    model: &mut Model,
    message_receiver: &mut UnboundedReceiver<Message>,
) -> Result<()> {
    terminal.draw(|frame| view(&model, frame))?;

    let mut current_msg = message_receiver.recv().await;

    while current_msg.is_some() {
        current_msg = update(model, current_msg.unwrap()).await?;
    }

    Ok(())
}

pub async fn run_tui(application: Application) -> Result<()> {
    let (message_sender, mut message_receiver) = mpsc::unbounded_channel();

    let lunch_start = DateTime::parse_from_rfc3339("2024-06-03T12:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let lunch_end = DateTime::parse_from_rfc3339("2024-06-03T13:30:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let meeting_start_time = DateTime::parse_from_rfc3339("2024-06-03T16:30:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let meeting_end_time = DateTime::parse_from_rfc3339("2024-06-03T17:30:00Z")
        .unwrap()
        .with_timezone(&Utc);

    let events = vec![Event {
        id: "blah".to_string(),
        title: "Lunch".to_string(),
        description: None,
        start_time: lunch_start,
        end_time: lunch_end,
    },
    Event {
        id: "sttez".to_string(),
        title: "Important meeting".to_string(),
        description: Some("Very important meeting with lots of important people. Very important things will be discussed. It is very important a conclusion is reached".to_string()),
        start_time: meeting_start_time,
        end_time: meeting_end_time,
    }];
    let day_events = vec![DayEvent {
        id: "scwhag".to_string(),
        title: "Birthday".to_string(),
        description: Some("Wish happy birthday".to_string()),
        date: Local::now().naive_local().date(),
    }];

    let mut terminal = init_terminal()?;
    let mut model = Model {
        application,
        current_state: CurrentState::MonthView,
        message_channel: message_sender.clone(),
        events_state: EventsState::Loading,
    };

    let cancellation_token = CancellationToken::new();
    let event_thread = handle_event(&model, message_sender.clone(), cancellation_token.clone());

    let now = Local::now();
    let today_midnight = now.with_time(NaiveTime::MIN).unwrap();
    let tomorrow_midnight = today_midnight + Duration::days(2);
    run_fetch_events_task(today_midnight, tomorrow_midnight, &model);

    loop {
        match main_loop(&mut terminal, &mut model, &mut message_receiver).await {
            Ok(_) => {
                if matches!(model.current_state, CurrentState::Done) {
                    cancellation_token.cancel();
                    event_thread.await?;

                    restore_terminal()?;

                    return Ok(());
                }
            }
            Err(e) => {
                cancellation_token.cancel();
                event_thread.await?;

                return Err(e);
            }
        };
    }
}
