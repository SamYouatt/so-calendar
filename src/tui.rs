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

#[derive(Default)]
struct Model {
    running_state: RunningState,
}

#[derive(Default, Debug, PartialEq, Eq)]
enum RunningState {
    #[default]
    Calendar,
    NewAccountModal,
    Done,
}

enum Message {
    OpenNewAccountModal,
    CloseModal,

    Quit,
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::OpenNewAccountModal => {}
        Message::CloseModal => todo!(),
        Message::Quit => model.running_state = RunningState::Done,
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

fn handle_event(_model: &Model) -> Result<Option<Message>> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            return Ok(handle_key(key));
        }
    }

    Ok(None)
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

pub fn run_tui() -> Result<()> {
    // create terminal here
    let mut terminal = init_terminal()?;
    let mut model = Model::default();

    while model.running_state != RunningState::Done {
        terminal.draw(|frame| view(&model, frame))?;

        let mut current_msg = handle_event(&model)?;

        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    restore_terminal()
}
