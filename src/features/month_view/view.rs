use chrono::Local;
use ratatui::prelude::*;
use ratatui::style::palette::tailwind;
use ratatui::widgets::*;

use crate::features::day_summary::components::day_summary_widget::DaySummaryWidget;
use crate::tui::model::EventsState;

pub fn render(frame: &mut Frame, events_state: &EventsState) {
    let (events, day_events) = match events_state {
        EventsState::Ready(events, day_events) => (events, day_events),
        _ => todo!(),
    };

    let main_layout =
        Layout::horizontal([Constraint::Fill(3), Constraint::Fill(1)]).split(frame.size());

    let month_block = Block::default().style(Style::new().bg(tailwind::STONE.c200));
    let month_view_placeholder = Paragraph::new("month view").block(month_block);

    let today_widget = DaySummaryWidget::new(Local::now().date_naive(), events, day_events);

    frame.render_widget(month_view_placeholder, main_layout[0]);
    frame.render_widget(today_widget, main_layout[1]);
}
