use chrono::{DateTime, Local};

use crate::{domain::events::{DayEvent, Event}, util::calendar_helpers::days_in_month};

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
