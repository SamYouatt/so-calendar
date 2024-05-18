use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::enable_raw_mode;
use crossterm::{
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    widgets::Paragraph,
    Frame, Terminal,
};
use std::io::stdout;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

type MessageSender = mpsc::UnboundedSender<Message>;

struct Model {
    running_state: CurrentState,
    message_sender: MessageSender,
}

#[derive(Debug, PartialEq, Eq)]
enum CurrentState {
    Calendar,
    NewAccountModal,

    // The app should close
    Done,
}

#[derive(Debug)]
enum Message {
    OpenNewAccountModal,
    CloseModal,
    Quit,
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::OpenNewAccountModal => {}
        Message::CloseModal => todo!(),
        Message::Quit => model.running_state = CurrentState::Done,
    };

    None
}

fn view(_model: &Model, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::THICK);
    let widget = Paragraph::new("SoCalendar").centered().block(block);

    frame.render_widget(widget, frame.size());
}

fn handle_event(
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

            let message = if event::poll(tick_rate).unwrap() {
                if let Event::Key(key) = event::read().expect("Failed to read crossterm event") {
                    if key.kind == event::KeyEventKind::Press {
                        handle_key(key)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
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

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('a') => Some(Message::OpenNewAccountModal),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub async fn run_tui() -> Result<()> {
    let (message_sender, mut message_receiver) = mpsc::unbounded_channel();

    let mut terminal = init_terminal()?;
    let mut model = Model {
        running_state: CurrentState::Calendar,
        message_sender,
    };

    let cancellation_token = CancellationToken::new();
    let event_thread = handle_event(
        &model,
        model.message_sender.clone(),
        cancellation_token.clone(),
    );

    loop {
        terminal.draw(|frame| view(&model, frame))?;

        let mut current_msg = message_receiver.recv().await;

        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }

        if model.running_state == CurrentState::Done {
            break;
        }
    }

    cancellation_token.cancel();
    event_thread.await?;

    restore_terminal()
}
