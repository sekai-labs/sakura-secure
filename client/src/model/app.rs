pub enum AppState {
    Main,
    SshConnect,
}

pub struct App {
    pub state: AppState,
    pub ssh_host: String,
    pub ssh_user: String,
    pub ssh_pass: String,
    pub status_msg: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::Main,
            ssh_host: "127.0.0.1:22".into(),
            ssh_user: "user".into(),
            ssh_pass: "pass".into(),
            status_msg: "".into(),
        }
    }
}
