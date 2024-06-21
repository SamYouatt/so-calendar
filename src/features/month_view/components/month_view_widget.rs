use crate::domain::events::{DayEvent, Event};

pub struct MonthViewWidget<'a> {
    events: &'a [Event],
    day_events: &'a [DayEvent],
}

impl<'a> MonthViewWidget<'a> {
    pub fn new(events: &'a [Event], day_events: &'a [DayEvent]) -> Self {
        Self {
            events,
            day_events,
        }
    }
}
