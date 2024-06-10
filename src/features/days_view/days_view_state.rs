use crate::domain::events::{DayEvent, Event};

#[derive(Debug)]
pub struct DaysViewState {
    pub events: Vec<Event>,
    pub day_events: Vec<DayEvent>,
}
