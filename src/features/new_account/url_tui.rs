use std::io::{self, stdout, Stdout};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, HighlightSpacing, List, ListDirection, ListItem, ListState},
};

#[derive(Debug, PartialEq)]
pub enum RunningState {
    Running,
    SelectionMade(LoginOption),
    Exited,
}

pub enum Message {
    Next,
    Previous,
    Select,
    Quit,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoginOption {
    OpenBrowser,
    CopyToClipboard,
}

#[derive(Copy, Clone)]
struct LoginOptionItem<'a> {
    text: &'a str,
    option: LoginOption,
}

impl<'a> From<LoginOptionItem<'a>> for ListItem<'a> {
    fn from(val: LoginOptionItem<'a>) -> Self {
        val.text.into()
    }
}

pub struct Model<'a> {
    list_state: ListState,
    items: Vec<LoginOptionItem<'a>>,
    pub state: RunningState,
}

impl<'a> Model<'a> {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let options = vec![
            LoginOptionItem {
                text: "Open browser",
                option: LoginOption::OpenBrowser,
            },
            LoginOptionItem {
                text: "Copy link to clipboard",
                option: LoginOption::CopyToClipboard,
            },
        ];

        Model {
            list_state,
            items: options,
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
            let selected_state = model
                .list_state
                .selected()
                .expect("Not possible to have an unselected item");
            let selected_option = model
                .items
                .get(selected_state)
                .expect("Not possible to have an unselected item");
            model.state = RunningState::SelectionMade(selected_option.option)
        }
        Message::Quit => {
            model.state = RunningState::Exited;
        }
    }
}

pub fn view(model: &mut Model, f: &mut Frame) {
    let list = List::new(model.items.clone())
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
    terminal.clear()?;
    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}
