use std::io::{self, stdout, Stdout};

use crossterm::{execute, terminal::{self, disable_raw_mode, enable_raw_mode}};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, HighlightSpacing, List, ListDirection, ListItem, ListState},
};

#[derive(Debug, PartialEq)]
pub enum RunningState {
    Running,
    SelectionMade(usize),
    Exited,
}

pub enum Message {
    Next,
    Previous,
    Select,
    Quit,
}

pub struct Model {
    list_state: ListState,
    items: Vec<String>,
    pub state: RunningState,
}

impl Model {
    pub fn new(items: &[String]) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Model {
            list_state,
            items: items.to_vec(),
            state: RunningState::Running,
        }
    }
}

pub fn update(model: &mut Model, msg: Message) {
    match msg {
        Message::Next => {
            let i = match model.list_state.selected() {
                Some(i) => {
                    if i >= model.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };

            model.list_state.select(Some(i));
        }
        Message::Previous => {
            let i = match model.list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        model.items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };

            model.list_state.select(Some(i));
        }
        Message::Select => {
            model.state = RunningState::SelectionMade(model.list_state.selected().unwrap())
        }
        Message::Quit => {
            model.state = RunningState::Exited;
        }
    }
}

pub fn view(model: &mut Model, f: &mut Frame) {
    let list_items: Vec<_> = model
        .items
        .iter()
        .map(|element| ListItem::new(element.clone()))
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .title("Link account")
                .borders(Borders::LEFT),
        )
        .style(Style::default().fg(Color::Magenta))
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::ITALIC),
        )
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    f.render_stateful_widget(&list, f.size(), &mut model.list_state);
}

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init_terminal() -> io::Result<Tui> {
    enable_raw_mode()?;
    Terminal::with_options(
        CrosstermBackend::new(stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(3),
        },
    )
}

pub fn restore_terminal(terminal: &mut Tui) -> io::Result<()> {
    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}
