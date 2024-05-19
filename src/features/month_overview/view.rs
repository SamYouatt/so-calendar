use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(75), Constraint::Min(22)])
        .split(frame.size());

    let calendar_block = Block::default()
        .title("SoCalendar")
        .borders(Borders::ALL)
        .border_set(border::THICK);
    let month_view_placeholder = Paragraph::new("Month view")
        .centered()
        .block(calendar_block);

    let today_block = Block::default()
        .title("19-May")
        .borders(Borders::ALL)
        .border_set(border::THICK);
    let today_view_placeholder = Paragraph::new("Today overview")
        .centered()
        .block(today_block);

    frame.render_widget(month_view_placeholder, main_layout[0]);
    frame.render_widget(today_view_placeholder, main_layout[1]);
}
