use std::net::TcpStream;

use ssh2::Session;

pub struct SshConnectState {
    pub host: String,
    pub username: String,
    pub password: String,
    pub focus_index: usize,
}

impl Default for SshConnectState {
    fn default() -> Self {
        Self {
            host: "127.0.0.1:22".into(),
            username: String::new(),
            password: String::new(),
            focus_index: 0,
        }
    }
}
pub struct SshSession {
    pub host: String,
    pub username: String,
    pub tcp: TcpStream,
    pub session: Session,
    pub output: String,
}

pub struct TerminalState {
    pub sessions: Vec<SshSession>,
}

impl Default for TerminalState {
    fn default() -> Self {
        Self {
            sessions: Vec::new(),
        }
    }
}
