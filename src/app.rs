use crate::{
    components::{
        colors::ColorComponent,
        image::{CustomImage, ImageState, ImageStruct},
        input_bar::Input,
    },
    helpers::config::Config,
    pages::{
        apply_page::{ApplyPage, ApplyTui, ImageData},
        image_input::{ImageInputPage, ImageInputTui},
        warning_page::WarningPage,
    },
    traits::get_input::DefaultInputComponent,
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct App {
    pub tx: Sender<Tui>,
    rx: Receiver<Tui>,
    exit: bool,
    current_page: Pages,
    image_page: ImageInputPage,
    apply_page: ApplyPage,
    warning_page: WarningPage,
    cfg: Config,
}

pub enum Tui {
    Event(Event),
    ChangePage(Pages),
    ImagePage(ImageInputTui),
    ApplyPage(ApplyTui),
}

pub enum Pages {
    Image,
    Apply(ImageData),
    WarningPage,
}

impl App {
    pub fn new(cfg: Config) -> Self {
        let (tx, rx) = mpsc::channel::<Tui>(10);
        Self {
            exit: false,
            current_page: if cfg.is_first_time() {
                Pages::WarningPage
            } else {
                Pages::Image
            },
            image_page: ImageInputPage::new(tx.clone()),
            apply_page: ApplyPage::new(tx.clone(), cfg.clone()),
            warning_page: WarningPage::new(),
            cfg,
            tx,
            rx,
        }
    }

    fn handle_event(&mut self, event: Tui) {
        match event {
            Tui::Event(event) => {
                match event {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        match key_event.code {
                            KeyCode::Char(ch) if key_event.modifiers == KeyModifiers::CONTROL => {
                                match ch {
                                    'q' => {
                                        self.exit = true;
                                        return;
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                match self.current_page {
                    Pages::Apply(_) => {
                        self.apply_page.handle_event(event.into());
                    }
                    Pages::Image => {
                        self.image_page.handle_event(event.into());
                    }
                    Pages::WarningPage => {}
                }
            }
            Tui::ImagePage(event) => {
                self.image_page.handle_event(event);
            }
            Tui::ChangePage(page) => {
                match &page {
                    Pages::Apply(data) => {
                        self.apply_page.set_data(data.clone());
                    }
                    Pages::Image => {}
                    Pages::WarningPage => {}
                }
                self.current_page = page;
            }
            Tui::ApplyPage(event) => {
                self.apply_page.handle_event(event);
            }
        }
    }

    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> Result<(), ()> {
        let mut reader = crossterm::event::EventStream::new();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame)).unwrap();
            tokio::select! {
                key_event = reader.next().fuse() => {
                   self.handle_event(Tui::Event(key_event.unwrap().unwrap()));
                },
                event = self.rx.recv().fuse() => {
                    self.handle_event(event.unwrap());
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        match self.current_page {
            Pages::Apply(_) => {
                self.apply_page.draw(frame);
            }
            Pages::Image => {
                self.image_page.draw(frame);
            }
            Pages::WarningPage => {
                self.warning_page.draw(frame);
            }
        }
    }
}
