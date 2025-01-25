# Sakura Secure

Sakura Secure is a terminal-based application (TUI) designed for managing SSH connections. It combines a sleek, user-friendly interface with powerful SSH capabilities to help users interact with remote servers more efficiently. The application also features a secure vault that stores SSH keys and synchronizes them across multiple devices, ensuring seamless access to remote systems from anywhere.

## Features
- **Terminal User Interface (TUI):** Intuitive layout for navigation and interaction.
- **SSH Connection Management:** Connect to remote servers using username and password.
- **Keybindings:**
  - `q`: Quit the application.
  - `h`: Open the help menu (placeholder).
  - `c`: Connect to an SSH server (placeholder).
- **Customizable Layout:** Modular design for easy extension.

## Prerequisites
- **Rust** (latest stable version) installed on your system. You can install it using [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## Installation
1. Clone this repository:
   ```bash
   git clone https://github.com/sekai-labs/sakura-secure.git
   cd sakura-secure
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run
   ```

## Usage
- On startup, the application displays a bordered block with a title.
- Use the keybindings to navigate:
  - Press `q` to quit.
  - Press `h` to open the help menu (future implementation).
  - Press `c` to simulate connecting to an SSH server (future implementation).
  
## License
This project is licensed under the [MIT License](LICENSE).

---

Developed with ❤️ by [sekai-labs](https://github.com/sekai-labs).