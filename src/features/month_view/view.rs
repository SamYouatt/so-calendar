use chrono::{Local, NaiveDate};
use ratatui::prelude::*;
use ratatui::style::palette::tailwind;
use ratatui::symbols::border;
use ratatui::widgets::*;

struct TodayWidget {
    date: NaiveDate,
}

impl Widget for TodayWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let main_container = Block::default()
            .padding(Padding::horizontal(1))
            .style(Style::new().bg(tailwind::STONE.c100));
        let main_container_area = main_container.inner(area);
        main_container.render(area, buf);


        let main_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(main_container_area);

        let formatted_date = self.date.format("%d %B, %A").to_string();
        Paragraph::new(formatted_date)
            .style(Style::new().fg(tailwind::RED.c500).bold())
            .render(main_layout[0], buf);

        let separator: String = std::iter::repeat("-").take(area.width.into()).collect();
        Paragraph::new(separator)
            .style(Style::new().fg(tailwind::STONE.c400))
            .render(main_layout[1], buf);
    }
}

pub fn render(frame: &mut Frame) {
    let main_layout =
        Layout::horizontal([Constraint::Fill(3), Constraint::Fill(1)]).split(frame.size());

    let month_block = Block::default().style(Style::new().bg(tailwind::STONE.c200));
    let month_view_placeholder = Paragraph::new("month view").block(month_block);

    let today_widget = TodayWidget {
        date: Local::now().date_naive(),
    };

    frame.render_widget(month_view_placeholder, main_layout[0]);
    frame.render_widget(today_widget, main_layout[1]);
}
