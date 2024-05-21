use ratatui::layout::{Constraint, Direction, Layout, Rect};

// Create a rect centered in the screen, with size of x-y pixels
pub fn centered_popup(r: Rect, x: u16, y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(y),
            Constraint::Min(0),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(x),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1]
}
