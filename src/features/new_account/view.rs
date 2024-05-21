use crate::configuration::Application;
use crate::tui::util::centered_popup;

use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(frame: &mut Frame, application: &Application) {
    // Need to render the account page first to render selections as popup over them
    crate::features::account_overview::view::render(frame, application);

    let centered_rect = centered_popup(frame.size(), 20, 5);

    let block = Block::bordered();
    let placeholder = Paragraph::new("sign in options").centered().block(block);

    frame.render_widget(placeholder, centered_rect);
}
