use ratatui::symbols::border;
use ratatui::Frame;
use ratatui::widgets::*;

use super::model::Model;

pub fn view(_model: &Model, frame: &mut Frame) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::THICK);
    let widget = Paragraph::new("SoCalendar").centered().block(block);

    frame.render_widget(widget, frame.size());
}
