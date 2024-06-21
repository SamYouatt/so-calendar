use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::style::palette::tailwind;

// 10:00 <-- slightly greyed out colour
//
// 11:00
//
// 12:00
fn render_times(frame: &mut Frame, area: Rect) {
    let area = Rect::new(area.x, area.y + 1, area.width, area.height - 1);

    // TODO: these should be passed in
    let time_from = 9;
    let time_until = 19;

    let num_lines = time_until - time_from;

    let constraints = (0..num_lines).map(|_| Constraint::Length(1));

    let layout = Layout::vertical(constraints).spacing(1).split(area);

    for (i, time) in (time_from..time_until).enumerate() {
        let formatted_time = format!("{:02}:00", time);
        let o_clock_line = Line::styled(formatted_time, tailwind::STONE.c400);
        frame.render_widget(o_clock_line, layout[i]);
    }
}


// xxxxxxxxxxxxxxxxxxxx  <- aiming for 20 characters wide
//
//       6 Jun (Red)
// 10:00 Team meeting     <-- some colour background
//       ------------
// 11:00 ------------     <-- different background to show it's this time currently
//       ------------
// 12:00
//
// 13:00 Blah blah        <-- some overlapping events
//       ---- Other..     <-- but this one starts later
// 14:00 ---- .......
fn render_day(frame: &mut Frame, area: Rect) {

    let date = Line::styled("6 Jun", (tailwind::RED.c400, Modifier::BOLD));
    frame.render_widget(date, area);

}

pub fn render(frame: &mut Frame) {
    let main_layout =
        Layout::horizontal([Constraint::Length(6), Constraint::Length(20), Constraint::Min(20)]).split(frame.size());

    let calendar_block = Block::default();
    let _month_view_placeholder = Paragraph::new("Days view").centered().block(calendar_block);

    let today_block = Block::default().borders(Borders::LEFT);
    let today_view_placeholder = Paragraph::new("Today overview")
        .centered()
        .block(today_block);

    //frame.render_widget(month_view_placeholder, main_layout[0]);
    render_times(frame, main_layout[0]);
    render_day(frame, main_layout[1]);
    frame.render_widget(today_view_placeholder, main_layout[2]);
}
