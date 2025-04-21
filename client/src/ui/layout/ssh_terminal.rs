use std::net::TcpStream;

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};
use ssh2::Session;

use crate::{
    app::ssh::try_ssh_connect,
    model::app::{App, AppState},
    ui::component::input::Input,
};

pub fn draw_ssh_connect_layout(frame: &mut Frame, app: &mut App) {
    let size = frame.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(size);

    let ssh = &mut app.ssh_connect;

    frame.render_widget(
        Paragraph::new("SSH Connect")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Magenta)),
        layout[0],
    );

    Input::new("Host", &mut ssh.host, ssh.focus_index == 0, false).render(frame, layout[1]);

    Input::new("Username", &mut ssh.username, ssh.focus_index == 1, false).render(frame, layout[2]);

    Input::new("Password", &mut ssh.password, ssh.focus_index == 2, true).render(frame, layout[3]);

    frame.render_widget(
        Paragraph::new("Press [Enter] to Connect")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Green)),
        layout[4],
    );
}

pub fn handle_ssh_connect_input(app: &mut App, code: KeyCode) {
    let ssh = &mut app.ssh_connect;

    match code {
        KeyCode::Tab => {
            ssh.focus_index = (ssh.focus_index + 1) % 3;
        }
        KeyCode::Backspace => match ssh.focus_index {
            0 => {
                ssh.host.pop();
            }
            1 => {
                ssh.username.pop();
            }
            2 => {
                ssh.password.pop();
            }
            _ => {}
        },
        KeyCode::Char(c) => match ssh.focus_index {
            0 => ssh.host.push(c),
            1 => ssh.username.push(c),
            2 => ssh.password.push(c),
            _ => {}
        },
        KeyCode::Enter => {
            if app.terminal_state.sessions.len() >= 4 {
                app.status_msg = "Maximum 4 sessions allowed".into();
                return;
            }

            match try_ssh_connect(&ssh.host, &ssh.username, &ssh.password) {
                Ok(output) => {
                    app.status_msg = format!("SSH connection successful: {}", output);
                    let ssh_session = crate::model::ssh::SshSession {
                        host: ssh.host.clone(),
                        username: ssh.username.clone(),
                        tcp: TcpStream::connect(&ssh.host).unwrap(),
                        session: Session::new().unwrap(),
                        output: String::new(),
                    };
                    app.terminal_state.sessions.push(ssh_session);
                    let idx = app.terminal_state.sessions.len() - 1;
                    app.state = AppState::SshTerminal(idx);
                }
                Err(err) => {
                    app.status_msg = format!("SSH connection failed: {}", err);
                }
            }
        }

        _ => {}
    }
}

pub fn draw_ssh_terminal_layout(frame: &mut Frame, app: &mut App, index: usize) {
    let size = frame.area();
    let session = &app.terminal_state.sessions[index];

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(size);
    frame.render_widget(
        Paragraph::new(format!(
            "Connected to: {}@{}",
            session.username, session.host
        ))
        .style(Style::default().fg(Color::Green)),
        layout[0],
    );

    // Display terminal area
    frame.render_widget(
        Paragraph::new("Terminal Window\nPress 'q' to disconnect, ESC to return to main menu")
            .style(Style::default().fg(Color::White)),
        layout[1],
    );
}

pub fn handle_ssh_terminal_input(app: &mut App, code: KeyCode, index: usize) {
    match code {
        KeyCode::Char('q') => {
            // Disconnect and remove the session
            app.terminal_state.sessions.remove(index);
            app.state = AppState::Main;
            app.status_msg = "Disconnected from SSH session".to_string();
        }
        KeyCode::Esc => {
            app.state = AppState::Main;
        }
        _ => {}
    }
}
