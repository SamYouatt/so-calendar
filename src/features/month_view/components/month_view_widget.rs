use chrono::{DateTime, Local};
use ratatui::prelude::*;
use ratatui::style::palette::tailwind;
use ratatui::widgets::*;

use crate::{
    domain::events::{DayEvent, Event},
    util::calendar_helpers::days_in_month,
};

pub struct MonthViewWidget<'a> {
    month: DateTime<Local>,
    events: &'a [Event],
    day_events: &'a [DayEvent],
    days_in_month: u32,
}

impl<'a> MonthViewWidget<'a> {
    pub fn new(month: DateTime<Local>, events: &'a [Event], day_events: &'a [DayEvent]) -> Self {
        Self {
            month,
            events,
            day_events,
            days_in_month: days_in_month(month),
        }
    }
}

impl Widget for MonthViewWidget<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let main_container = Block::default()
            .style(Style::new().bg(tailwind::STONE.c200));
        Paragraph::new("shiny month view")
            .block(main_container)
            .render(area, buf);
    }
}
