use ratatui::symbols::border;
use ratatui::widgets::*;
use ratatui::prelude::*;

pub fn render(frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::THICK);
    let widget = Paragraph::new("SoCalendar").centered().block(block);

    frame.render_widget(widget, frame.size());
}
