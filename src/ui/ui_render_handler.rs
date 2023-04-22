use crate::models::state_model::{App, InputMode};
use std::vec;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use super::ui_text::{editing_text, render_copyright, render_typing};

//chunk array
// 0 = top text
// 1 = input box
// 2 = messages
// 3 = copyright
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Percentage(83),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    let (msg, style) = match app.input_mode {
        InputMode::Editing => editing_text(),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = render_typing(app);
    f.render_widget(input, chunks[1]);

    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
    let mut cursor_rendering = |chunk: usize| {
        f.set_cursor(
            // Put cursor past the end of the input text
            chunks[chunk].x + app.input.width() as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[chunk].y + 1,
        )
    };

    cursor_rendering(1);

    let results: Vec<ListItem> = app
        .results
        .records
        .iter()
        .map(|m| {
            let content = vec![Spans::from(Span::raw(format!(
                "name: {} id: {}",
                m.Name, m.Id
            )))];
            ListItem::new(content)
        })
        .collect();

    let results: List = List::new(results)
        .block(
            Block::default()
                .style(Style::default().fg(Color::Blue))
                .borders(Borders::ALL)
                .title("sObjects"),
        )
        .style(Style::default().fg(Color::LightCyan))
        .highlight_style(
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        );

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            // let m: String = serde_json::from_str(&m.body).unwrap();
            let content = vec![Spans::from(Span::raw(format!(
                "{}",
                m.body.replace('"', "")
            )))];
            ListItem::new(content)
        })
        .collect();
    let messages: List = List::new(messages)
        .style(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .style(Style::default().fg(Color::Blue))
                .borders(Borders::ALL)
                .title("Results and Fields"),
        )
        .highlight_style(
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        );

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(50)].as_ref())
        .split(chunks[2]);
    match app.input_mode {
        InputMode::Editing => {
            f.render_stateful_widget(results, middle_chunks[0], &mut app.results.state);
            f.render_widget(messages, middle_chunks[1]);
            // f.render_stateful_widget(connections, middle_chunks[1], &mut app.connections.state);
        }
    }

    let copyright = render_copyright();
    f.render_widget(copyright, chunks[3]);
}
