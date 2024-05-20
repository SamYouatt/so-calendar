use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::block::Title;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame) {
    let main_block = Block::bordered()
        .border_set(border::THICK)
        .title(Title::from("Accounts").alignment(Alignment::Center));

    let placeholder = Paragraph::new("Accounts").block(main_block);

    frame.render_widget(placeholder, frame.size());
}
