use color_eyre::owo_colors::{colors::css::Grey, OwoColorize};
use crossterm::{event::{KeyCode, KeyEvent}, style::Color};
use ratatui::{
    style::Stylize, text::Line, widgets::{Block, BorderType, Paragraph, Widget}
};

use crate::traits::get_input::InputComponent;

pub struct Button {
    callback: Option<Box<dyn Fn() -> ()>>,
    title: Option<String>,
    focused: bool,
}

impl Button {
    pub fn new() -> Self {
        Button {
            callback: None,
            title: None,
            focused: false
        }
    }
    pub fn set_title<T: ToString>(mut self, title: T) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn change_title<T: ToString>(&mut self, title: T) {
        self.title = Some(title.to_string())
    }

    pub fn set_callback<T: Fn() -> () + 'static>(mut self, callback: T) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }
}

impl InputComponent for Button {
    fn handle_key_event(&mut self, key_event: &crossterm::event::KeyEvent) {
        let mut call = false;
        match key_event.code {
            KeyCode::Char(ch) if ch == ' ' => {
                call = true;
            }
            KeyCode::Enter => {
                call = true;
            }
            _ => {}
        }
        if let Some(callback) = &self.callback {
            if call {
                callback();
            }
        }
    }
    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
}

impl Widget for &Button {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut paragraph = Paragraph::new(self.title.clone().unwrap_or("".to_string()))
            .block(Block::bordered().border_type(BorderType::Rounded))
            .centered();
        if self.focused {
            paragraph = paragraph.reversed();
        }
        paragraph.render(area, buf);
    }
}
