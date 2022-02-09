use tui::{
    layout::{Alignment},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders,Paragraph,
    },
};
use crate::services;

pub fn render_home<'a>() -> Paragraph<'a> {
    let _contr = services::contributors::get_contributors();
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "TRINCI Database Explorer",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}