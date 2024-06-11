use ratatui::prelude::*;
use ratatui::style::palette::tailwind;
use ratatui::widgets::Widget;
use ratatui::widgets::*;

use crate::domain::events::DayEvent;
use crate::util::text_helpers::truncate_text;

pub struct DayEventWidget<'a> {
    event: &'a DayEvent,
}

impl<'a> DayEventWidget<'a> {
    pub fn new(event: &'a DayEvent) -> Self {
        Self { event }
    }
}

impl Widget for DayEventWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let event_row_layout = Layout::horizontal([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(7),
        ])
        .spacing(1)
        .split(area);

        // DayEvent dot indicator
        Paragraph::new("●")
            .style(Style::new().fg(tailwind::EMERALD.c500).bold())
            .render(event_row_layout[0], buf);

        // Main event information
        let max_width: usize = event_row_layout[1].width.into();

        let event_title = truncate_text(&self.event.title, max_width);
        Paragraph::new(event_title)
            .style(Style::new().fg(tailwind::STONE.c800))
            .render(event_row_layout[1], buf);

        // All day indicator
        Paragraph::new("All day")
            .style(Style::new().fg(tailwind::STONE.c500))
            .render(event_row_layout[2], buf);
    }
}
