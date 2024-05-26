use crate::configuration::Application;
use crate::tui::util::centered_popup;

use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame, application: &Application) {
    // Need to render the account page first to render selections as popup over them
    crate::features::account_overview::view::render(frame, application);

    let centered_rect = centered_popup(frame.size(), 30, 6);

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

    frame.render_widget(options_block, centered_rect);
    frame.render_widget(title, options_layout[0]);
    frame.render_widget(list, options_layout[1]);
}
