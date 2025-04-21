use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub struct Input<'a> {
    pub title: &'a str,
    pub value: &'a mut String,
    pub is_focused: bool,
    pub hidden: bool,
}

impl<'a> Input<'a> {
    pub fn new(title: &'a str, value: &'a mut String, is_focused: bool, hidden: bool) -> Self {
        Self {
            title,
            value,
            is_focused,
            hidden,
        }
    }

    pub fn render(self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let display_value = if self.hidden {
            "*".repeat(self.value.len())
        } else {
            self.value.to_string()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title)
            .style(if self.is_focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        let input = Paragraph::new(display_value)
            .block(block)
            .alignment(Alignment::Left)
            .style(Style::default().fg(Color::White));

        frame.render_widget(input, area);
    }

    pub fn handle_input<F: FnMut(&str)>(&mut self, key: KeyEvent, mut callback: F) {
        if !self.is_focused {
            return;
        }
        match key.code {
            KeyCode::Char(c) => self.value.push(c),
            KeyCode::Backspace => {
                self.value.pop();
            }
            KeyCode::Enter => {
                callback(&self.value.clone());
            }
            _ => {}
        }
    }
}
