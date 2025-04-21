use std::io;

use chrono::Local;
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::model::app::{App, AppState};

pub fn draw_main_layout(frame: &mut Frame) {
    let size = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    let main_block = Block::default()
        .title(" Sakura Secure ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Magenta));
    frame.render_widget(main_block, size);

    let header = Paragraph::new("Welcome to Sakura Secure")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    let now = Local::now();
    let time_str = now.format("%Y-%m-%d %H:%M").to_string();
    let clock = Paragraph::new(Span::from(Span::raw(time_str)))
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Right);
    frame.render_widget(clock, chunks[2]);
}

pub fn handle_main_key(app: &mut App, code: KeyCode) -> io::Result<()> {
    match code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('s') => app.state = AppState::SshConnect,
        _ => {}
    }
    Ok(())
}
