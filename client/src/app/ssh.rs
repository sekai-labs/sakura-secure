use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

pub fn try_ssh_connect(host: &str, user: &str, pass: &str) -> Result<String, String> {
    let tcp = TcpStream::connect(host).map_err(|e| format!("TCP failed: {}", e))?;
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session
        .handshake()
        .map_err(|e| format!("Handshake failed: {}", e))?;
    session
        .userauth_password(user, pass)
        .map_err(|e| format!("Auth failed: {}", e))?;

    if session.authenticated() {
        let mut channel = session
            .channel_session()
            .map_err(|e| format!("Session failed: {}", e))?;
        channel
            .exec("uname -a")
            .map_err(|e| format!("Exec failed: {}", e))?;

        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap_or_default();
        channel.wait_close().unwrap();

        Ok(s)
    } else {
        Err("Authentication failed".into())
    }
}
