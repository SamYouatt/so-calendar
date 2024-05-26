use std::io::stdout;

use color_eyre::eyre::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::configuration::Application;

use self::handle_event::handle_event;
use self::update::update;
use self::view::view;
use self::{
    model::{CurrentState, Message, Model},
    task_manager::{TaskKey, TaskManager},
};

pub(crate) mod handle_event;
pub(crate) mod model;
pub(crate) mod task_manager;
pub(crate) mod update;
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

pub async fn run_tui(application: Application) -> Result<()> {
    let (message_sender, mut message_receiver) = mpsc::unbounded_channel();
    let mut task_manager = TaskManager::new();

    let mut terminal = init_terminal()?;
    let mut model = Model {
        application,
        current_state: CurrentState::MonthView,
        message_channel: message_sender.clone(),
        task_manager: &mut task_manager,
    };

    model
        .task_manager
        .run_task(TaskKey::EventHandler, move |cancellation_token| {
            handle_event(message_sender.clone(), cancellation_token);
        });

    loop {
        terminal.draw(|frame| view(&model, frame))?;

        let mut current_msg = message_receiver.recv().await;

        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap())?;
        }

        if model.current_state == CurrentState::Done {
            break;
        }
    }

    model.task_manager.cancel_all().await;

    restore_terminal()
}
