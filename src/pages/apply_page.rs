use std::time::Duration;

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
use tokio::{sync::mpsc::{self, Receiver, Sender}, time::sleep};

pub struct ApplyPage {
    pub continue_button: Button,
    pub tx: Sender<Tui>,
    pub focused: FocusTracker,
    pub completed: bool,
    pub image: Option<ImageData>,
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
            focused: FocusTracker::new(vec![1]),
            completed: false,
            image: None,
            cfg,
        }
    }

    pub fn set_data(&mut self, data: ImageData) {
        self.image = Some(data)
    }

    pub fn get_component(&mut self, pos: (u64, u64)) -> &mut dyn DefaultInputComponent {
        match pos.0 {
            0 => match pos.1 {
                0 => &mut self.continue_button as &mut dyn DefaultInputComponent,
                _ => &mut self.continue_button as &mut dyn DefaultInputComponent,
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
                self.cfg.process(&self.image.as_ref().unwrap().colors);
                self.continue_button.change_title("Done!");
                let tx = self.tx.clone();
                tokio::task::spawn(async move {
                    sleep(Duration::from_secs(2)).await;
                    tx.send(ApplyTui::Delayed.into()).await.unwrap();
                });
                self.completed = true;
            },
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
                Constraint::Fill(1),
                Constraint::Max(3),
            ])
            .split(frame.area());
        let top_bar = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(4), Constraint::Fill(1)])
            .split(layout[0]);
        let mid = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
            .split(layout[1]);
        let instructions = Line::from(vec!["CTRL + Q".blue().reversed(), ":Quit".into()]);
        let ix = Paragraph::new(instructions)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .alignment(Alignment::Center);

        frame.render_widget(&self.continue_button, *top_bar.last().unwrap());
        frame.render_widget(
            Paragraph::new(image.image_path.clone()).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Selected image:"),
            ),
            *top_bar.first().unwrap(),
        );
        frame.render_widget(ix, *layout.last().unwrap());

        let files = self.cfg
                .get_files();
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
        frame.render_widget(from_block, mid[0]);
        frame.render_widget(destination_block, mid[1]);
        // self.image_ui.render_image(frame, layout[1]);
        // frame.render_widget(&self.colors, layout[2]);
    }
}
