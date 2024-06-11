use chrono::{Local, NaiveDate};
use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::style::palette::tailwind;
use ratatui::symbols::border;
use ratatui::widgets::*;

use crate::domain::events::Event;
use crate::tui::model::EventsState;

struct EventWidget<'a> {
    event: &'a Event,
}

impl<'a> EventWidget<'a> {
    pub fn new(event: &'a Event) -> Self {
        Self { event }
    }
}

impl Widget for EventWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let event_row_layout = Layout::horizontal([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(5),
        ])
        .spacing(1)
        .split(area);

        // Event dot indicator
        Paragraph::new("●")
            .style(Style::new().fg(tailwind::SKY.c500).bold())
            .render(event_row_layout[0], buf);

        // Main event information
        let event_content_layout = Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
            .split(event_row_layout[1]);

        Paragraph::new(self.event.title.to_string())
            .style(Style::new().fg(tailwind::STONE.c800))
            .render(event_content_layout[0], buf);

        if let Some(description) = &self.event.description {
            let mut description = description.to_owned();
            let max_length: usize = event_content_layout[1].width.into();

            if description.len() >= max_length {
                description = format!("{}…", &description[..max_length - 1].trim());
            }

            Paragraph::new(description.as_str())
                .style(Style::new().fg(tailwind::STONE.c500))
                .render(event_content_layout[1], buf);
        }

        // Event time information
        let event_time_layout = Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
            .split(event_row_layout[2]);

        let start_time = self.event.start_time.format("%R").to_string();
        Paragraph::new(start_time)
            .style(Style::new().fg(tailwind::STONE.c700))
            .render(event_time_layout[0], buf);
        let end_time = self.event.end_time.format("%R").to_string();
        Paragraph::new(end_time)
            .style(Style::new().fg(tailwind::STONE.c500))
            .render(event_time_layout[1], buf);
    }
}

struct TodayWidget<'a> {
    date: NaiveDate,
    events: &'a [Event],
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

        let main_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(main_container_area);

        let formatted_date = self.date.format("%d %B, %A").to_string();
        Paragraph::new(formatted_date)
            .style(Style::new().fg(tailwind::RED.c500).bold())
            .render(main_layout[0], buf);

        let separator: String = std::iter::repeat("-").take(area.width.into()).collect();
        Paragraph::new(separator)
            .style(Style::new().fg(tailwind::STONE.c400))
            .render(main_layout[1], buf);

        let events_layout_constraints =
            std::iter::repeat(Constraint::Length(2)).take(self.events.len());
        let events_layout = Layout::vertical(events_layout_constraints)
            .flex(Flex::Start)
            .split(main_layout[2]);

        for (i, event) in self.events.iter().enumerate() {
            EventWidget::new(event).render(events_layout[i], buf);
        }
    }
}

pub fn render(frame: &mut Frame, events_state: &EventsState) {
    let (events, _day_events) = match events_state {
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
    };

    frame.render_widget(month_view_placeholder, main_layout[0]);
    frame.render_widget(today_widget, main_layout[1]);
}
