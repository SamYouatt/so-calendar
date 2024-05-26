use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::block::Title;
use ratatui::widgets::*;

use crate::configuration::Application;

use super::retrieve_accounts::retrieve_accounts;

pub async fn render(frame: &mut Frame<'_>, application: &Application) {
    let accounts = retrieve_accounts(application).await.unwrap();

    let main_block = Block::bordered()
        .border_set(border::THICK)
        .title(Title::from("Accounts").alignment(Alignment::Center));

    let account_list = List::new(accounts.iter().map(|acc| acc.email.to_string()))
        .block(main_block)
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .direction(ListDirection::TopToBottom);

    frame.render_widget(account_list, frame.size());
}
