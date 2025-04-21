use crate::app::tui::Tui;
use crossterm::event::{self, Event, KeyCode};
use model::app::{App, AppState};
use std::{error::Error, io};
use ui::layout::{
    menu::{draw_main_layout, handle_main_key},
    ssh_terminal::{
        draw_ssh_connect_layout, draw_ssh_terminal_layout, handle_ssh_connect_input,
        handle_ssh_terminal_input,
    },
};

mod app;
mod handler;
mod helper;
mod model;
mod service;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut tui = Tui::new()?;
    tui.init()?;
    let result = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = result {
        println!("Error: {}", err);
    }

    Ok(())
}
fn run_app(tui: &mut Tui) -> io::Result<()> {
    let mut app = App::default();

    loop {
        tui.terminal.draw(|frame| {
            match app.state {
                AppState::Main => draw_main_layout(frame),
                AppState::SshConnect => draw_ssh_connect_layout(frame, &mut app),
                AppState::SshTerminal(index) => draw_ssh_terminal_layout(frame, &mut app, index),
            };
        })?;

        if let Event::Key(key) = event::read()? {
            match app.state {
                AppState::Main => {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                    handle_main_key(&mut app, key.code)?;
                }
                AppState::SshConnect => {
                    if key.code == KeyCode::Esc {
                        app.state = AppState::Main;
                        continue;
                    }
                    handle_ssh_connect_input(&mut app, key.code);
                }
                AppState::SshTerminal(index) => {
                    if key.code == KeyCode::Esc {
                        app.state = AppState::Main;
                        continue;
                    }
                    handle_ssh_terminal_input(&mut app, key.code, index);
                }
            }
        }

        if let AppState::Main = app.state {
            if app.should_quit {
                break;
            }
        }
    }
    Ok(())
}
