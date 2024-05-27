use crate::configuration::Application;
use crate::tui::util::centered_popup;

use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render_sign_in_options(frame: &mut Frame, application: &Application, selected_index: usize) {
    // Need to render the account page first to render selections as popup over them
    crate::features::account_overview::view::render(frame, application);

    let centered_rect = centered_popup(frame.size(), 35, 6);

    let options_block = Block::bordered();
    let options_block_inner = options_block.inner(centered_rect);
    let options_layout = Layout::vertical([Constraint::Length(2), Constraint::Length(2)])
        .flex(Flex::SpaceBetween)
        .split(options_block_inner);

    let title = Paragraph::new("Log in").centered();

    let list = List::new(vec!["Open browser", "Copy link to clipboard"])
        .style(Style::default().fg(Color::Magenta))
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::ITALIC),
        )
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    let mut list_state = ListState::default();
    list_state.select(Some(selected_index));

    frame.render_widget(options_block, centered_rect);
    frame.render_widget(title, options_layout[0]);
    frame.render_stateful_widget(list, options_layout[1], &mut list_state);
}

pub fn render_waiting_for_signin(frame: &mut Frame, application: &Application) {
    // Need to render the account page first to render selections as popup over them
    crate::features::account_overview::view::render(frame, application);

    let centered_rect = centered_popup(frame.size(), 35, 6);

    let block = Block::bordered();
    let block_inner = block.inner(centered_rect);
    let block_layout = Layout::vertical([Constraint::Length(2), Constraint::Length(2)])
        .flex(Flex::SpaceBetween)
        .split(block_inner);

    let title = Paragraph::new("Log in").centered();

    let waiting_message = Paragraph::new("Waiting for you to sign in...").centered();

    frame.render_widget(block, centered_rect);
    frame.render_widget(title, block_layout[0]);
    frame.render_widget(waiting_message, block_layout[1]);
}
