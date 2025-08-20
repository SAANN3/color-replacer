use crossterm::event::{KeyCode, KeyEvent};

pub enum Axis {
    Horizontal(Horizontal),
    Vertical(Vertical),
    Empty,
}
pub enum Horizontal {
    Left,
    Right,
}

pub enum Vertical {
    Up,
    Down,
}

pub fn get_axis(key_event: &KeyEvent) -> Axis {
    match key_event.code {
        KeyCode::Left => Axis::Horizontal(Horizontal::Left),
        KeyCode::Right => Axis::Horizontal(Horizontal::Right),
        KeyCode::Up => Axis::Vertical(Vertical::Up),
        KeyCode::Down => Axis::Vertical(Vertical::Down),
        _ => Axis::Empty,
    }
}
pub trait InputComponent{
    fn set_focused(&mut self, focused: bool);
    fn handle_key_event(&mut self, key_event: &KeyEvent);
    fn keep_focus_x(&mut self, axis: &Horizontal) -> bool {
        false
    }
    fn keep_focus_y(&mut self, axis: &Vertical) -> bool {
        false
    }
}

pub trait DefaultInputComponent: InputComponent {
    fn handle_key_event_focused(&mut self, key_event: &KeyEvent) -> (bool, Axis) {
        let axis = get_axis(key_event);
        let state = match &axis {
            Axis::Horizontal(axis) => self.keep_focus_x(axis),
            Axis::Vertical(axis) => self.keep_focus_y(axis),
            Axis::Empty => false,
        };
        self.handle_key_event(key_event);
        (state, axis)
    }
}

impl<T> DefaultInputComponent for T where T: InputComponent {}
