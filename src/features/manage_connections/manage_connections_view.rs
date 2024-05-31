use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::widgets::*;

use super::manage_connections_state::ManageConnectionsState;

pub fn render(state: &ManageConnectionsState, frame: &mut Frame<'_>) {
    let selected_account = &state.accounts[state.selected_account_index];

    let accounts = state.accounts.iter().map(|acc| acc.email.to_string());
    let filtered_calendars = state
        .calendars
        .iter()
        .filter(|cal| cal.account_id == selected_account.id)
        .map(|cal| cal.title.to_string());

    let mut account_list_state = ListState::default();
    account_list_state.select(Some(state.selected_account_index));

    let vertical_split =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.size());

    let accounts_block = Block::bordered()
        .border_type(BorderType::Thick)
        .title(Title::from("Accounts").alignment(Alignment::Left));

    let calendars_block = Block::bordered()
        .border_type(BorderType::Thick)
        .title(Title::from("Calendars").alignment(Alignment::Left));

    let account_list = List::new(accounts)
        .block(accounts_block)
        .highlight_symbol(">>")
        .direction(ListDirection::TopToBottom);

    let calendar_list = List::new(filtered_calendars)
        .block(calendars_block)
        .highlight_symbol(">>")
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(account_list, vertical_split[0], &mut account_list_state);
    frame.render_widget(calendar_list, vertical_split[1]);
}
