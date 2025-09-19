use std::{num::Saturating, time::Duration};

use crate::{
    app::{Pages, Tui},
    components::{
        button::Button,
        colors::ColorComponent,
        colors_picker::ColorPicker,
        image::{CustomImage, ImageState, ImageStruct},
        input_bar::Input,
    },
    helpers::config::{Config, ReplaceColors},
    traits::{
        focus_tracker::FocusTracker,
        get_input::{get_axis, DefaultInputComponent, InputComponent},
    },
};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin}, style::Stylize, symbols, text::{Line, Text}, widgets::{Block, BorderType, Paragraph, Widget}, DefaultTerminal, Frame
};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    time::sleep,
};

pub struct ApplyPage {
    pub continue_button: Button,
    pub tx: Sender<Tui>,
    pub focused: FocusTracker,
    pub completed: bool,
    pub image: Option<ImageData>,
    pub selected_colors: ReplaceColors<ColorPicker>,
    pub color_component: ColorComponent,
    pub cfg: Config,
}

#[derive(Clone)]
pub struct ImageData {
    pub image_path: String,
    pub colors: Vec<String>,
}

pub enum ApplyTui {
    Event(Event),
    ContinueButton(),
    Delayed,
}

impl From<ApplyTui> for Tui {
    fn from(value: ApplyTui) -> Self {
        Tui::ApplyPage(value)
    }
}

impl From<Event> for ApplyTui {
    fn from(value: Event) -> Self {
        ApplyTui::Event(value)
    }
}

impl ApplyPage {
    pub fn new(tx: Sender<Tui>, cfg: Config) -> Self {
        Self {
            continue_button: {
                let tx = tx.clone();
                let mut btn = Button::new().set_title("Replace").set_callback(move || {
                    let tx = tx.clone();
                    tokio::task::spawn(async move {
                        tx.send(ApplyTui::ContinueButton().into()).await.unwrap();
                    });
                });
                btn.set_focused(true);
                btn
            },
            tx,
            focused: FocusTracker::new(vec![1, 1, 1, 1]),
            completed: false,
            image: None,
            color_component: ColorComponent::new(),
            cfg,
            selected_colors: ReplaceColors {
                primary: ColorPicker::new(Vec::new()),
                secondary: ColorPicker::new(Vec::new()),
                tertiary: ColorPicker::new(Vec::new()),
            },
        }
    }

    pub fn set_data(&mut self, data: ImageData) {
        self.color_component.set_colors(data.colors.clone());
        self.selected_colors = ReplaceColors {
            primary: ColorPicker::new(data.colors.clone())
                .set_title("Primary  ".into())
                .with_pos(0),
            secondary: ColorPicker::new(data.colors.clone())
                .set_title("Secondary".into())
                .with_pos(1),
            tertiary: ColorPicker::new(data.colors.clone())
                .set_title("Tertiary ".into())
                .with_pos(2),
        };
        self.image = Some(data);
    }

    pub fn get_component(&mut self, pos: (u64, u64)) -> &mut dyn DefaultInputComponent {
        match pos.0 {
            0 => match pos.1 {
                _ => &mut self.continue_button as &mut dyn DefaultInputComponent,
            },
            1 => match pos.1 {
                _ => &mut self.selected_colors.primary as &mut dyn DefaultInputComponent,
            },
            2 => match pos.1 {
                _ => &mut self.selected_colors.secondary as &mut dyn DefaultInputComponent,
            },
            3 => match pos.1 {
                _ => &mut self.selected_colors.tertiary as &mut dyn DefaultInputComponent,
            },
            _ => &mut self.continue_button as &mut dyn DefaultInputComponent,
        }
    }

    pub fn handle_event(&mut self, event: ApplyTui) {
        match event {
            ApplyTui::Event(event) => match event {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    let change = self
                        .get_component(self.focused.current())
                        .handle_key_event_focused(&key_event);
                    if !change.0 {
                        let res = self.focused.change_focused(get_axis(&key_event));
                        if !res.is_same() {
                            self.get_component(res.old).set_focused(false);
                            self.get_component(res.new).set_focused(true);
                        }
                    }
                }
                _ => {}
            },
            ApplyTui::ContinueButton() => {
                self.cfg.process(&ReplaceColors {
                    primary: self.selected_colors.primary.get_color(),
                    secondary: self.selected_colors.secondary.get_color(),
                    tertiary: self.selected_colors.tertiary.get_color(),
                });
                self.continue_button.change_title("Done!");
                let tx = self.tx.clone();
                tokio::task::spawn(async move {
                    sleep(Duration::from_secs(2)).await;
                    tx.send(ApplyTui::Delayed.into()).await.unwrap();
                });
                self.completed = true;
            }
            ApplyTui::Delayed => {
                self.continue_button.change_title("Replace");
            }
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let image = self.image.as_ref().unwrap();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(3),
                Constraint::Max(5),
                Constraint::Fill(1),
                Constraint::Max(3),
                Constraint::Max(3),
            ])
            .split(frame.area());

        let colors_bar = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Max(1),
            ])
            .split(layout[1].inner(Margin::new(1, 1)));
        frame.render_widget(
            Block::bordered().border_type(BorderType::Rounded),
            layout[1],
        );
        frame.render_widget(&self.selected_colors.primary, colors_bar[0]);
        frame.render_widget(&self.selected_colors.secondary, colors_bar[1]);
        frame.render_widget(&self.selected_colors.tertiary, colors_bar[2]);

        let top_bar = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(4), Constraint::Fill(1)])
            .split(layout[0]);
        let mid = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
            .split(layout[2]);

        let instructions = Line::from(vec![
            "CTRL + Q".blue().reversed(), 
            ":Quit | ".into(),
            "+(=)/-".blue().reversed(),
            ":Lighten/Darken color | ".into(),
            "R".blue().reversed(), 
            ":Reset color | ".into(),
            "Arrow keys to navigate".blue().reversed()

        ]);
        let ix = Paragraph::new(instructions)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .alignment(Alignment::Center);
        frame.render_widget(ix, *layout.last().unwrap());

        frame.render_widget(
            Paragraph::new(image.image_path.clone()).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Selected image:"),
            ),
            *top_bar.first().unwrap(),
        );

        let files = self.cfg.get_files();
        let from_block = Paragraph::new(Text::from(
            files
                .iter()
                .map(|x| x.from.to_str().to_owned().unwrap().into())
                .collect::<Vec<Line<'_>>>(),
        ))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Detected files:"),
        );
        let destination_block = Paragraph::new(Text::from(
            files
                .iter()
                .map(|x| x.to.to_str().to_owned().unwrap().into())
                .collect::<Vec<Line<'_>>>(),
        ))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Destination:"),
        );
        let current_colors = Paragraph::new(Text::from(vec!["A".into(), "B".into()])).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Chosen colors"),
        );
        frame.render_widget(&self.continue_button, *top_bar.last().unwrap());
        frame.render_widget(&self.color_component, layout[3]);
        frame.render_widget(from_block, mid[0]);
        frame.render_widget(destination_block, mid[1]);
        // self.image_ui.render_image(frame, layout[1]);
        // frame.render_widget(&self.colors, layout[2]);
    }
}
