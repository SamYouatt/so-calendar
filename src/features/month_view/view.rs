use chrono::Local;
use ratatui::prelude::*;

use crate::features::day_summary::components::day_summary_widget::DaySummaryWidget;
use crate::tui::model::EventsState;

use super::components::month_view_widget::MonthViewWidget;

pub fn render(frame: &mut Frame, events_state: &EventsState) {
    let (events, day_events) = match events_state {
        EventsState::Ready(events, day_events) => (events, day_events),
        _ => return,
    };

    let main_layout =
        Layout::horizontal([Constraint::Fill(3), Constraint::Fill(1)]).split(frame.size());

    let current_month = Local::now();
    let month_view_widget = MonthViewWidget::new(current_month, events, day_events);

    let today_widget = DaySummaryWidget::new(Local::now().date_naive(), events, day_events);

    frame.render_widget(month_view_widget, main_layout[0]);
    frame.render_widget(today_widget, main_layout[1]);
}
