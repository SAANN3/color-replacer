use crate::{
    app::{Pages, Tui},
    components::{
        button::Button,
        colors::ColorComponent,
        image::{CustomImage, ImageState},
        input_bar::Input,
    },
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
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use tokio::sync::mpsc::{self, Receiver, Sender};

use super::apply_page::ImageData;

pub struct ImageInputPage {
    pub image_path: Input,
    pub image_ui: CustomImage,
    pub colors: ColorComponent,
    pub continue_button: Button,
    pub tx: Sender<Tui>,
    pub focused: FocusTracker,
}

pub enum ImageInputTui {
    Event(Event),
    UpdateImage(ImageState),
    PathChanged(String),
    ContinueButton(),
}

impl From<ImageInputTui> for Tui {
    fn from(value: ImageInputTui) -> Self {
        Tui::ImagePage(value)
    }
}

impl From<Event> for ImageInputTui {
    fn from(value: Event) -> Self {
        ImageInputTui::Event(value)
    }
}

impl ImageInputPage {
    pub fn new(tx: Sender<Tui>) -> Self {
        Self {
            image_path: {
                let tx = tx.clone();
                let mut input = Input::new()
                    .set_title("Image path:".into())
                    .set_callback(move |value: String| {
                        let tx = tx.clone();
                        tokio::task::spawn(async move {
                            tx.send(ImageInputTui::PathChanged(value).into())
                                .await
                                .unwrap();
                        });
                    })
                    .set_value("");
                input.set_focused(true);
                input
            },
            continue_button: {
                let tx = tx.clone();
                Button::new().set_title("Continue").set_callback(move || {
                    let tx = tx.clone();
                    tokio::task::spawn(async move {
                        tx.send(ImageInputTui::ContinueButton().into())
                            .await
                            .unwrap();
                    });
                })
            },
            image_ui: CustomImage::new(),
            colors: ColorComponent::new(),
            tx,
            focused: FocusTracker::new(vec![2]),
        }
    }

    pub fn get_component(&mut self, pos: (u64, u64)) -> &mut dyn DefaultInputComponent {
        match pos.0 {
            0 => match pos.1 {
                0 => &mut self.image_path as &mut dyn DefaultInputComponent,
                1 => &mut self.continue_button as &mut dyn DefaultInputComponent,
                _ => &mut self.image_path as &mut dyn DefaultInputComponent,
            },
            _ => &mut self.image_path as &mut dyn DefaultInputComponent,
        }
    }

    pub fn handle_event(&mut self, event: ImageInputTui) {
        match event {
            ImageInputTui::Event(event) => match event {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char(ch) if key_event.modifiers == KeyModifiers::CONTROL => {
                            match ch {
                                'h' => {
                                    self.colors.hex_colors = !self.colors.hex_colors;
                                    return;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
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
            ImageInputTui::ContinueButton() => {
                if let Some(x) = self.colors.get_data() {
                    self.tx.try_send(Tui::ChangePage(Pages::Apply(
                        ImageData {
                            image_path: self.image_path.get_value(),
                            colors: x,
                        }
                    ))).unwrap();
                } else {
                    self.continue_button.change_title("Select valid file");
                }
            }
            ImageInputTui::UpdateImage(state) => {
                self.image_ui.update_state(state);
            }
            ImageInputTui::PathChanged(path) => {
                self.image_ui.process_image(self.tx.clone(), path.clone());
                self.colors.set_path(path);
            }
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(3),
                Constraint::Fill(1),
                Constraint::Max(3),
                Constraint::Max(3),
            ])
            .split(frame.area());
        let top_bar = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(4), Constraint::Fill(1)])
            .split(layout[0]);
        let instructions = Line::from(vec![
            "CTRL + Q".blue().reversed(),
            ":Quit".into(),
            " ".into(),
            "CTRL + H:".blue().reversed(),
            {
                if self.colors.hex_colors {
                    "Hide hex".into()
                } else {
                    "Show hex".into()
                }
            },
        ]);
        let ix = Paragraph::new(instructions)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .alignment(Alignment::Center);

        frame.render_widget(&self.image_path, *top_bar.first().unwrap());
        frame.render_widget(&self.continue_button, *top_bar.last().unwrap());

        frame.render_widget(ix, *layout.last().unwrap());
        self.image_ui.render_image(frame, layout[1]);
        frame.render_widget(&self.colors, layout[2]);
    }
}
