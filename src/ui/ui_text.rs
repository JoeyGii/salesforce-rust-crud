use crate::models::state_model::{App, InputMode};
use chrono::Datelike;
use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
};
pub fn normal_text<'a>() -> (Vec<Span<'a>>, Style) {
    (
        vec![
            Span::raw("  📟 Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, Press "),
            Span::styled("l", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to login. This is walkie talkie wiggles. 📟"),
        ],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    )
}

pub fn editing_text<'a>() -> (Vec<Span<'a>>, Style) {
    (
        vec![
            Span::raw("  Press "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to stop editing, "),
            Span::styled(
                "Enter sObject first then name query: Account.Brenda",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" press enter when done"),
            Span::styled("~", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("  📟"),
        ],
        Style::default(),
    )
}
pub fn connection_text<'a>() -> (Vec<Span<'a>>, Style) {
    (
        vec![
            Span::raw("  Press "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to stop editing, "),
            Span::styled(
                "Enter names with a comma between and no spaces",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" to setup a new connection 📟"),
        ],
        Style::default(),
    )
}
pub fn render_copyright<'a>() -> Paragraph<'a> {
    let get_current_year = || -> String {
        let current_date = chrono::Utc::now();
        let year = current_date.year();
        year.to_string()
    };
    let copyright = Paragraph::new(format!(
        "📟 Wiggle-CLI {} - all rights reserved 📟 ",
        get_current_year()
    ))
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightBlue))
            .title("Copyright")
            .border_type(BorderType::Plain),
    );
    copyright
}

pub fn render_typing(app: &mut App) -> Paragraph {
    let rendered_text = &app.input;

    let input: Paragraph = Paragraph::new(rendered_text.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Have Fun :D")
                .style(Style::default().fg(Color::LightBlue)),
        );

    input
}
pub fn render_home<'a>(app: &App) -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Walkie 📟")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Talkie 📟")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("WIGGLES")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Remember to be nice to your friends.",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        match app.input_mode {
            InputMode::Editing => Spans::from(vec![Span::raw("")]),
        },
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Blue))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}
