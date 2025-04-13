use chrono::Local;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::model::app::App;

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

    // Draw the main border
    let main_block = Block::default()
        .title(" Sakura Secure ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Magenta));
    frame.render_widget(main_block, size);

    // Header block
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

pub fn draw_ssh_layout(frame: &mut Frame, app: &App) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Style},
        text::{Line, Span, Text},
        widgets::{Block, Borders, Paragraph},
    };

    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let block = Block::default()
        .title("SSH Connect")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(block, area);

    let host = Paragraph::new(Text::from(Line::from(Span::raw(format!(
        "Host: {}",
        app.ssh_host
    )))));
    frame.render_widget(host, chunks[0]);

    let user = Paragraph::new(Text::from(Line::from(Span::raw(format!(
        "User: {}",
        app.ssh_user
    )))));
    frame.render_widget(user, chunks[1]);

    let pass = Paragraph::new(Text::from(Line::from(Span::raw(format!(
        "Password: {}",
        app.ssh_pass
    )))));
    frame.render_widget(pass, chunks[2]);

    let status = Paragraph::new(Text::from(app.status_msg.clone()));
    frame.render_widget(status, chunks[3]);
}

pub fn draw_ssh_connect_layout(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Host
            Constraint::Length(3), // User
            Constraint::Length(3), // Pass
            Constraint::Length(3), // Submit
            Constraint::Min(0),    // Status
        ])
        .split(size);

    let header = Paragraph::new("SSH Connect")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Magenta));
    frame.render_widget(header, layout[0]);

    let host = Paragraph::new(format!("Host: {}", app.ssh_host))
        .block(Block::default().borders(Borders::ALL).title("Host"));
    frame.render_widget(host, layout[1]);

    let user = Paragraph::new(format!("User: {}", app.ssh_user))
        .block(Block::default().borders(Borders::ALL).title("Username"));
    frame.render_widget(user, layout[2]);

    let pass = Paragraph::new(format!("Password: {}", "*".repeat(app.ssh_pass.len())))
        .block(Block::default().borders(Borders::ALL).title("Password"));
    frame.render_widget(pass, layout[3]);

    let submit = Paragraph::new("Press [Enter] to Connect")
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center);
    frame.render_widget(submit, layout[4]);

    let status = Paragraph::new(Text::from(Line::from(Span::raw(app.status_msg.clone()))))
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(status, layout[5]);
}
