use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::traits::get_input::{Horizontal, InputComponent};
pub struct Input {
    value: String,
    cursor_pos: u64,
    title: Option<String>,
    on_change: Option<Box<dyn Fn(String)>>,
    focused: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            value: "".to_string(),
            cursor_pos: 0,
            title: None,
            on_change: None,
            focused: false,
        }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = Option::Some(title);
        self
    }

    pub fn set_value<T: ToString>(mut self, value: T) -> Self {
        let value = value.to_string();
        self.cursor_pos = value.len() as u64;
        self.change_value(value);
        self
    }

    pub fn set_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }
    fn split(&self) -> (&str, &str, &str) {
        let split_a = self
            .value
            .split_at_checked(self.cursor_pos as usize)
            .unwrap_or((&self.value, ""));
        let split_b = split_a.1.split_at_checked(1).unwrap_or(("", ""));
        let before = split_a.0;
        let mid = split_b.0;
        let after = split_b.1;
        return (before, mid, after);
    }

    fn move_left(&mut self) {
        self.cursor_pos = self.cursor_pos.checked_sub(1).unwrap_or(0);
    }

    fn move_right(&mut self) {
        self.cursor_pos = self.cursor_pos.checked_add(1).unwrap_or(0);
        if self.cursor_pos >= self.value.len() as u64 {
            self.cursor_pos = self.value.len() as u64;
        }
    }

    pub fn change_value(&mut self, value: String) {
        self.value = value;
        if let Some(callback) = &self.on_change {
            callback(self.value.clone());
        }
    }
}

impl InputComponent for Input {
    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
    fn handle_key_event(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char(char) => {
                let (before, mid, after) = self.split();
                self.change_value(before.to_owned() + &(char.to_string()) + mid + after);
                self.move_right();
            }
            KeyCode::Backspace => {
                let (before, mid, after) = self.split();
                let len = before.len().checked_sub(1).unwrap_or(0);
                let before = before
                    .split_at_checked(len)
                    .unwrap_or(("", ""))
                    .0
                    .to_owned();
                let str = before + mid + after;
                self.change_value(str);
                self.move_left();
            }
            KeyCode::Left => {
                self.move_left();
            }
            KeyCode::Right => {
                self.move_right();
            },
            _ => {}
        }
    }
    fn keep_focus_x(&mut self, axis: &Horizontal) -> bool {
        match axis {
            Horizontal::Right => {
                self.cursor_pos != self.value.len() as u64
            },
            Horizontal::Left => {
                self.cursor_pos != 0
            }
        }
    }
}

impl Widget for &Input {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let (before, mid, after) = self.split();
        let mid = if mid.is_empty() { " " } else { mid };
        let str = Line::from(vec![
            before.into(),
            if self.focused {
                mid.reversed()
            } else {
                mid.not_bold()
            }, 
            after.into()
        ]);
        let mut block = Block::bordered().border_type(ratatui::widgets::BorderType::Rounded);
        if let Some(title) = &self.title {
            block = block.title(title.clone());
        }
        let paragraph = Paragraph::new(str).block(block);
        paragraph.render(area, buf);
    }
}
