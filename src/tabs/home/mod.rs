use tui::{
    layout::{Alignment},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders,Paragraph,
    },
};
use crate::services;
fn contribs_to_span_map(c:&services::contributors::Contributor)->String {
    format!("{} ({})", c.name,c.url)
}
fn strings_to_span(s: String)->tui::text::Span {
    Span::raw(s)
}
pub fn render_home<'a>() -> Paragraph<'a> {
    let contribs = services::contributors::get_contributors();
    let contribs_spans:Vec<String> = contribs.iter().map(contribs_to_span_map).collect();
    let span_contributors = Spans::from(contribs_spans.iter().map(strings_to_span).collect());
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome to")]),
        Spans::from(vec![Span::styled(
            "TRINCI Database Explorer",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("contributors:")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(span_contributors)
        //Spans::from(vec![Span::raw(iterContrib.nth(0))]),
        //Spans::from(contribsSpans.iter().map(|s| Span::raw(s))),
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