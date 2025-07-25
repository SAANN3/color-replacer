use crate::{
    app::{Pages, Tui},
    components::{
        button::Button,
        colors::ColorComponent,
        image::{CustomImage, ImageState, ImageStruct},
        input_bar::Input,
    },
    helpers::config::Config,
    traits::{
        focus_tracker::FocusTracker,
        get_input::{get_axis, DefaultInputComponent, InputComponent},
    },
};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct WarningPage {}

impl WarningPage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Max(3),
            ])
            .split(frame.area());
        let example_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(layout[1]);
        let warning: Paragraph<'_> = Paragraph::new(Text::from(vec![
            "It seems that your config file is not yet configured, you need to edit it first"
                .into(),
            "If it your first time it will contain an example".into(),
            format!("Config file path: {:?}", Config::get_config_path()).into(),
            "Also set first_time to false in order to continue".into(),
        ]))
        .block(Block::bordered().border_type(BorderType::Rounded))
        .alignment(Alignment::Center);

        let example_in: Paragraph<'_> = Paragraph::new(Text::from(vec![
            format!("param1  = {}", Config::replace_key(0)).into(),
            "...".into(),
            format!("param16 = {}", Config::replace_key(15)).into(),
        ]))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Example of 'from' file"),
        );
        let example_out: Paragraph<'_> = Paragraph::new(Text::from(vec![
            format!("param1  = #ffffff").into(),
            "...".into(),
            format!("param16 = #030303").into(),
        ]))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Output ('to')"),
        );
        let instructions = Line::from(vec!["CTRL + Q".blue().reversed(), ":Quit".into()]);
        let ix = Paragraph::new(instructions)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .alignment(Alignment::Center);
        frame.render_widget(ix, *layout.last().unwrap());
        frame.render_widget(warning, *layout.first().unwrap());
        frame.render_widget(example_in, example_layout[0]);
        frame.render_widget(example_out, example_layout[1]);

    }
}
