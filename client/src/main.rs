use crossterm::event::{self, Event, KeyCode};
use model::app::{App, AppState};
use ui::layout::{draw_main_layout, draw_ssh_layout};

use crate::app::tui::Tui;
use std::{error::Error, io};

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
        tui.terminal.draw(|frame| match app.state {
            AppState::Main => draw_main_layout(frame),
            AppState::SshConnect => draw_ssh_layout(frame, &app),
        })?;

        if let Ok(event) = event::read() {
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('s') => {
                        app.state = AppState::SshConnect;
                    }
                    KeyCode::Char('m') => {
                        app.state = AppState::Main;
                    }
                    _ => {}
                }
            }
        }
    }
}
