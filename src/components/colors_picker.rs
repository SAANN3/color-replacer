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
    selected: SelectedColor,
    focused: bool,
    title: String,
}

struct SelectedColor {
    custom_color: Option<CustomColor>,
    pos: usize,
}

#[derive(Clone)]
struct CustomColor {
    modified: String,
    original: String,
    light: i8, // [-10 ; 10]
}

impl ColorPicker {
    pub fn new(colors: Vec<String>) -> Self {
        Self {
            colors,
            selected: SelectedColor {
                custom_color: None,
                pos: 0,
            },
            focused: false,
            title: String::new(),
        }
    }

    pub fn with_pos(mut self, pos: usize) -> Self {
        self.selected.pos = cmp::min(pos, self.colors.len());
        self
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn get_color(&self) -> String {
        if let Some(color) = &self.selected.custom_color {
            return color.modified.clone();
        }
        self.colors[self.selected.pos].clone()
    }

    fn get_custom_color(&mut self) -> &mut CustomColor {
        if self.selected.custom_color.is_none() {
            self.selected.custom_color = Some(CustomColor {
                light: 0,
                modified: self.get_color(),
                original: self.get_color(),
            });
        }
        self.selected.custom_color.as_mut().unwrap()

    }

    fn change_color_light(&mut self, light: i8) {
        let custom_color = self.get_custom_color();
        custom_color.light = light;
        match Color::from_str(&custom_color.original).unwrap() {
            Color::Rgb(r, g, b) => {
                let modifier = (custom_color.light + 10) as f64 / 10.0;
                let r = (r as f64 * modifier) as u8;
                let g = (g as f64 * modifier) as u8;
                let b = (b as f64 * modifier) as u8;

                let color = Color::Rgb(r, g, b).to_string();
                custom_color.modified = color;
            }
            _ => {
                // ummmmm lets suppose its rgb ok
                panic!("Color expected to be rgb");
            }
        }
    }


    pub fn set_pos(&mut self, pos: usize) {
        self.selected.pos = pos;
        if self.selected.custom_color.is_some() {
            self.selected.custom_color = None;
        };
    }

    pub fn lighten(&mut self) {
        let mut light = self.get_custom_color().light;
        light = cmp::min(light + 1, 10);
        self.change_color_light(light);
        
    }

    pub fn darken(&mut self) {
        let mut light = self.get_custom_color().light;
        light = cmp::max(light - 1, -10);
        self.change_color_light(light);

    }
}

impl InputComponent for ColorPicker {
    fn handle_key_event(&mut self, key_event: &crossterm::event::KeyEvent) {
        match key_event.code {
            KeyCode::Right => self.set_pos(cmp::min(self.selected.pos + 1, self.colors.len() - 1)),
            KeyCode::Left => self.set_pos(self.selected.pos.checked_sub(1).unwrap_or(0)),
            KeyCode::Char(ch) => match ch {
                '+' => {
                    self.lighten();
                }
                '-' => {
                    self.darken();
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn keep_focus_x(&mut self, axis: &crate::traits::get_input::Horizontal) -> bool {
        match axis {
            crate::traits::get_input::Horizontal::Left => {
                if self.selected.pos == 0 {
                    return false;
                }
                true
            }
            crate::traits::get_input::Horizontal::Right => {
                if self.selected.pos == self.colors.len() - 1 {
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
        text[self.selected.pos * 2] = "|".repeat(1).into();
        text[self.selected.pos * 2 + 2] = "|".repeat(1).into();
        if self.focused {
            text.insert(0, self.title.clone().reversed());
        } else {
            text.insert(0, self.title.clone().into())
        };
        text.insert(
            0,
            symbols::block::FULL
                .repeat(3)
                .fg(Color::from_str(&self.get_color()).unwrap()),
        );
        let text: Line = text.into();
        let paragraph = Paragraph::new(text);
        paragraph.render(area, buf);
    }
}
