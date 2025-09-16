
use std::fs;

use ratatui::{layout::{Constraint, Flex, Layout, Rect}, widgets::{Block, BorderType, Paragraph}, Frame};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol, Resize, StatefulImage};
use tokio::sync::mpsc::Sender;

use crate::{app::Tui, pages::image_input::ImageInputTui};


pub struct CustomImage {
    state: ImageState,
}

pub enum ImageState {
    WaitingForInput,
    Loading,
    FileNotFound,
    Unsupported,
    Loaded(ImageStruct),
}
pub struct ImageStruct {
    url: String,
    image: StatefulProtocol,
}

impl CustomImage {
    pub fn new() -> Self {
        CustomImage { 
            state: ImageState::WaitingForInput,
        }
    }

    pub fn is_loaded(&self) -> bool {
        match &self.state  {
            ImageState::Loaded(_) => true,
            _ => false
        }
    }

    pub fn process_image<T: ToString>(&mut self, rx: Sender<Tui>, path: T) {
        let path = path.to_string();
        
        // match &self.state {
        //     ImageState::Loading => return,
        //     ImageState::Loaded(data) if data.url == path => {
        //         return
        //     }
        //     _ => {}
        // }
        self.state = ImageState::Loading;
        tokio::spawn(async move {
            let dyn_img = image::ImageReader::open::<String>(path.clone());
            let result = if let Ok(dyn_img) = dyn_img  {
                if let Ok(dyn_img) = dyn_img.decode() {
                    let picker = Picker::from_query_stdio().unwrap();
                    ImageState::Loaded(ImageStruct { 
                        url: path,
                        image: picker.new_resize_protocol(dyn_img)
                    })
                } else {
                    ImageState::Unsupported
                }
            } else {
                ImageState::FileNotFound  
            };
            let _ = rx.send(ImageInputTui::UpdateImage(result).into()).await;
        });
    }

    pub fn update_state(&mut self, state: ImageState ) {
        self.state = state;
    }

    fn center(area: Rect, w: u16, h: u16) -> Rect {
        let horizontal = *Layout::horizontal([Constraint::Length(w)])
            .flex(Flex::Center)
            .split(area)
            .first()
            .unwrap();
        *Layout::vertical([Constraint::Length(h)])
            .flex(Flex::Center)
            .split(horizontal)
            .first()
            .unwrap()
            
    }

    fn centered_paragraph<T: ToString>(frame: &mut Frame, area: Rect, string: T) {
        let string = string.to_string();
        let area = Self::center(area, string.len() as u16, 1);
        frame.render_widget(
            Paragraph::new(string.clone()),
            area
        );  
    }

    pub fn render_image(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().border_type(BorderType::Rounded);
        let inner_area = block.inner(area);
        let image_widget = StatefulImage::default()
            .resize(Resize::Scale(None));
        frame.render_widget(block, area);
        match &mut self.state {
            ImageState::WaitingForInput => {
                Self::centered_paragraph(frame, inner_area, "Waiting for input...");
            },
            ImageState::Loading => {
                Self::centered_paragraph(frame, inner_area, "Loading...");
            },
            ImageState::FileNotFound => {
                Self::centered_paragraph(frame, inner_area, "File not found");
            },
            ImageState::Unsupported => {
                Self::centered_paragraph(frame, inner_area, "File unsupported");
            },
            ImageState::Loaded(data) => {
                let ImageStruct {image, .. } = data;
                let image_rect = image.size_for(Resize::Scale(None), area);
                frame.render_stateful_widget(image_widget,
                    Self::center(inner_area, image_rect.width, image_rect.height), 
                    image
                );
            },
        }        
    }
}

