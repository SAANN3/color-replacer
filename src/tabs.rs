// use std::iter;

// use crossterm::event::{KeyCode, KeyEvent};
// use ratatui::{
//     buffer::Buffer,
//     layout::{Constraint, Direction, Layout, Rect, Size},
//     style::{Color, Style, Stylize},
//     symbols::{self, border},
//     text::{Line, Text},
//     widgets::{block, Block, Borders, Paragraph, Scrollbar, Tabs, Widget},
//     DefaultTerminal, Frame,
// };
// use ratatui::prelude::StatefulWidget;
// use tui_scrollview::{ScrollView, ScrollViewState};

// use crate::traits::get_input::InputComponent;

// #[derive(Debug)]
// pub struct TabItems {
//     pub tabs: Vec<String>,
//     pub divider: String,
//     pub length: u16,
//     pub selected_tab: u8,
// }
// impl TabItems {
//     pub fn new(items: Vec<String>) -> Self {
//         let divider = "|".to_string();
//         let divider_len = divider.len() as u16;
//         let length = items.iter().map(|x| x.len() as u16 + divider_len).sum::<u16>() + 1;
//         Self {
//             tabs: items,
//             length: length,
//             selected_tab: 0,
//             divider: divider
//         }
//     }
//     pub fn forward(&mut self) {
//         let var = &mut self.selected_tab;
//         if *var + 1 < self.tabs.len() as u8 {
//             *var += 1;
//         } else {
//             *var = 0;
//         }
//     }
//     pub fn backwards(&mut self) {
//         self.selected_tab = self.selected_tab
//             .checked_sub(1)
//             .unwrap_or(self.tabs.len() as u8 - 1);
//     }

    
// }
// impl InputComponent for TabItems {
//     fn handle_key_event(&mut self, key_event: &KeyEvent) {
//         match key_event.code {
//             KeyCode::Tab => {
//                 self.forward();
//             }
//             _ => {}
//         }
//     }
// }
// impl Widget for &TabItems {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         let layout = Layout::new(
//             Direction::Horizontal,
//             vec![Constraint::Percentage(100)]   
//         ).split(Rect { x: area.x, y: area.y, width: self.length, height: area.width });

//         let tab = Tabs::default()
//             .titles(self.tabs.clone())
//             .block(Block::bordered().title("Tabs"))
//             .style(Style::default().white())
//             .highlight_style(Style::default().yellow())
//             .select(self.selected_tab as usize)
//             .padding("", "")
//             .divider(self.divider.clone());
//             // .render(area, buf);
//         let mut scroll_view = ScrollView::new(Size::new(self.length, area.height));
//         let mut state = ScrollViewState::default();
//         let mut pos = state.offset();
//         pos.x = self.selected_tab as u16 * 7;
//         state.set_offset(pos);
//         scroll_view.render_widget(tab, layout[0]);
//         scroll_view.render(area, buf,&mut  state);
//         // tab.render(layout[0], buf);
        
//     }
// }
