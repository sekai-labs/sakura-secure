use super::ssh::{SshConnectState, TerminalState};

#[derive(PartialEq, Eq)]
pub enum AppState {
    Main,
    SshConnect,
    SshTerminal(usize),
}

pub struct App {
    pub state: AppState,
    pub ssh_connect: SshConnectState,
    pub terminal_state: TerminalState,
    pub status_msg: String,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::Main,
            ssh_connect: SshConnectState::default(),
            terminal_state: TerminalState::default(),
            status_msg: "".into(),
            should_quit: false,
        }
    }
}
