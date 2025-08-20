use std::{cmp, str::FromStr, string};

use color_eyre::owo_colors::OwoColorize;
use crossterm::event::KeyCode;
use ratatui::{
    style::{Color, Stylize},
    symbols::{self, border},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph, Widget},
};
use ratatui_image::protocol::Protocol;

use crate::traits::{get_input::InputComponent, helpers::Separator};
pub struct ColorPicker {
    colors: Vec<String>,
    selected: usize,
    focused: bool,
    title: String,
}

impl ColorPicker {
    pub fn new(colors: Vec<String>) -> Self {
        Self {
            colors,
            selected: 0,
            focused: false,
            title: String::new(),
        }
    }

    pub fn with_pos(mut self, pos: usize) -> Self {
        self.selected = cmp::min(pos, self.colors.len());
        self
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn get_color(&self) -> String {
        self.colors.get(self.selected).unwrap().to_string()
    }
}

impl InputComponent for ColorPicker {
    fn handle_key_event(&mut self, key_event: &crossterm::event::KeyEvent) {
        match key_event.code {
            KeyCode::Right => {
                self.selected = cmp::min(self.selected + 1, self.colors.len() - 1);
            }
            KeyCode::Left => self.selected = self.selected.checked_sub(1).unwrap_or(0),
            _ => {}
        }
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn keep_focus_x(&mut self, axis: &crate::traits::get_input::Horizontal) -> bool {
        match axis {
            crate::traits::get_input::Horizontal::Left => {
                if self.selected == 0 {
                    return false;
                }
                true
            }
            crate::traits::get_input::Horizontal::Right => {
                if self.selected == self.colors.len() - 1 {
                    return false;
                }
                true
            }
        }
    }
}

impl Widget for &ColorPicker {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut text = self
            .colors
            .iter()
            .map(|x| {
                let color = Color::from_str(x).unwrap();
                symbols::block::FULL.repeat(3).fg(color)
            })
            .collect::<Vec<Span<'static>>>()
            .separate(" ".into());
        text[self.selected * 2] = "|".repeat(1).into();
        text[self.selected * 2 + 2] = "|".repeat(1).into();
        if self.focused {
            text.insert(0, self.title.clone().reversed());
        } else {
            text.insert(0, self.title.clone().into())
        };
        text.insert(
            0,
            symbols::block::FULL
                .repeat(3)
                .fg(Color::from_str(&self.colors[self.selected]).unwrap()),
        );
        let text: Line = text.into();
        let paragraph = Paragraph::new(text);
        paragraph.render(area, buf);
    }
}
