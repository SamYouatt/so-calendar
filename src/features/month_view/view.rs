use ratatui::prelude::*;
use ratatui::style::palette::tailwind;
use ratatui::symbols::border;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame) {
    let main_layout =
        Layout::horizontal([Constraint::Fill(1), Constraint::Length(30)]).split(frame.size());

    let month_block = Block::default().style(Style::new().bg(tailwind::STONE.c200));
    let month_view_placeholder = Paragraph::new("month view").block(month_block);

    let today_block = Block::default().style(Style::new().bg(tailwind::STONE.c100));
    let today_view_placeholder = Paragraph::new("Today overview")
        .centered()
        .block(today_block);

    frame.render_widget(month_view_placeholder, main_layout[0]);
    frame.render_widget(today_view_placeholder, main_layout[1]);
}
