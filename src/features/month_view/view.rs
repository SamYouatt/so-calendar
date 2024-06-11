use chrono::{Local, NaiveDate};
use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::style::palette::tailwind;
use ratatui::widgets::*;

use crate::domain::events::{DayEvent, Event};
use crate::tui::model::EventsState;
use crate::util::text_helpers::truncate_text;


struct TodayWidget<'a> {
    date: NaiveDate,
    events: &'a [Event],
    day_events: &'a [DayEvent],
}

impl Widget for TodayWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let main_container = Block::default()
            .padding(Padding::horizontal(1))
            .style(Style::new().bg(tailwind::STONE.c100));
        let main_container_area = main_container.inner(area);
        main_container.render(area, buf);

        let num_day_events: u16 = self
            .day_events
            .len()
            .try_into()
            .expect("way too many day events");

        let main_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(num_day_events + 1),
            Constraint::Fill(1),
        ])
        .split(main_container_area);

        // Selected date
        let formatted_date = self.date.format("%d %B, %A").to_string();
        Paragraph::new(formatted_date)
            .style(Style::new().fg(tailwind::RED.c500).bold())
            .render(main_layout[0], buf);

        // Separator
        let separator: String = std::iter::repeat("-").take(area.width.into()).collect();
        Paragraph::new(separator)
            .style(Style::new().fg(tailwind::STONE.c400))
            .render(main_layout[1], buf);

        // Day events list
        let day_events_layout_constraints =
            std::iter::repeat(Constraint::Length(1)).take(num_day_events.into());
        let events_layout = Layout::vertical(day_events_layout_constraints)
            .flex(Flex::Start)
            .split(main_layout[2]);

        for (i, day_event) in self.day_events.iter().enumerate() {
            DayEventWidget::new(day_event).render(events_layout[i], buf);
        }

        // Events list
        let events_layout_constraints =
            std::iter::repeat(Constraint::Length(2)).take(self.events.len());
        let events_layout = Layout::vertical(events_layout_constraints)
            .flex(Flex::Start)
            .split(main_layout[3]);

        for (i, event) in self.events.iter().enumerate() {
            EventWidget::new(event).render(events_layout[i], buf);
        }
    }
}

pub fn render(frame: &mut Frame, events_state: &EventsState) {
    let (events, day_events) = match events_state {
        EventsState::Ready(events, day_events) => (events, day_events),
        _ => todo!(),
    };

    let main_layout =
        Layout::horizontal([Constraint::Fill(3), Constraint::Fill(1)]).split(frame.size());

    let month_block = Block::default().style(Style::new().bg(tailwind::STONE.c200));
    let month_view_placeholder = Paragraph::new("month view").block(month_block);

    let today_widget = TodayWidget {
        date: Local::now().date_naive(),
        events,
        day_events,
    };

    frame.render_widget(month_view_placeholder, main_layout[0]);
    frame.render_widget(today_widget, main_layout[1]);
}
