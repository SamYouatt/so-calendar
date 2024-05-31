use std::fmt::Display;

use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::widgets::*;

use super::manage_connections_state::ManageConnectionsState;
use super::Account;

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email)
    }
}

pub fn render(state: &ManageConnectionsState, frame: &mut Frame<'_>) {
    let vertical_split =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.size());

    let accounts_block = Block::bordered()
        .border_type(BorderType::Thick)
        .title(Title::from("Accounts").alignment(Alignment::Left));

    let calendars_block = Block::bordered()
        .border_type(BorderType::Thick)
        .title(Title::from("Calendars").alignment(Alignment::Left));

    let account_list = List::new(state.accounts.iter().map(|acc| acc.email.to_string()))
        .block(accounts_block)
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .direction(ListDirection::TopToBottom);

    let calendar_list = List::new(state.calendars.iter().map(|cal| cal.title.to_string()))
        .block(calendars_block)
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .direction(ListDirection::TopToBottom);

    let mut account_list_state = ListState::default();
    account_list_state.select(Some(state.selected_account_index));

    frame.render_stateful_widget(account_list, vertical_split[0], &mut account_list_state);
    frame.render_widget(calendar_list, vertical_split[1]);
}
