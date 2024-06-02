use super::handle_days_view_message::Event;

#[derive(Debug)]
pub struct DaysViewState {
    pub events: Vec<Event>,
}
