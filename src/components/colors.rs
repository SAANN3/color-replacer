use std::{str::FromStr, string};

use color_eyre::owo_colors::OwoColorize;
use ratatui::{style::{Color, Stylize}, symbols, text::{Line, Span}, widgets::{Block, Paragraph, Widget}};
use ratatui_image::protocol::Protocol;

use crate::traits::helpers::Separator;
pub struct ColorComponent {
    state: Option<State>,
    pub hex_colors: bool,
}

enum State {
    ERROR,
    COLORS(Vec<String>),
}

impl ColorComponent {
    pub fn new() -> Self {
        Self {
            state: None,
            hex_colors: false,
        }
    }

    pub fn get_data(&self) -> Option<Vec<String>> {
        if let Some(state) = &self.state {
            match state {
                State::COLORS(x) => Some(x.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn set_path<T: ToString>(&mut self, path: T) {
        let path = path.to_string();
        let colors = image_palette::load(&path);
        self.state = if colors.is_ok() {
            Some(State::COLORS(
                colors
                    .unwrap()
                    .iter()
                    .map(|x| x.color().to_string())
                    .collect::<Vec<String>>(),
            ))
        } else {
            Some(State::ERROR)
        }
    }
}

impl Widget for &ColorComponent {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let text: Line = if let Some(state) = &self.state {
            match state {
                State::COLORS(colors) => {
                    let separator = Span::default().content("|");
                    if !self.hex_colors {
                        colors.iter()
                            .map(|x| {
                                let color = Color::from_str(x).unwrap();
                                symbols::block::FULL.repeat(3).fg(color)
                            })
                            .collect::<Vec<Span<'static>>>()
                            .separate(separator)
                            .into()
                    }
                    else {
                        colors.iter()
                            .map(|x| { 
                                let color = Color::from_str(x).unwrap();
                                x.clone().fg(color)
                            })
                            .collect::<Vec<Span<'static>>>()
                            .separate(separator)
                            .into()
                    } 
                        
                },
                State::ERROR => {
                    "Error occured".into()
                }
            }
        } else {
            "".into()
        };
        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title("Colors"))
            .centered()
            ;
        paragraph.render(area, buf);
    }
}
