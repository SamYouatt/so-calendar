use ratatui::prelude::*;
use ratatui::style::palette::tailwind;
use ratatui::widgets::Widget;
use ratatui::widgets::*;

use crate::domain::events::Event;
use crate::util::text_helpers::truncate_text;

pub struct EventWidget<'a> {
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
        let max_width: usize = event_content_layout[1].width.into();

        let event_title = truncate_text(&self.event.title, max_width);
        Paragraph::new(event_title)
            .style(Style::new().fg(tailwind::STONE.c800))
            .render(event_content_layout[0], buf);

        if let Some(description) = &self.event.description {
            Paragraph::new(truncate_text(description, max_width))
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
