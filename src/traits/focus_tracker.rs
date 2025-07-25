use super::get_input::{Axis, Horizontal, Vertical};

pub struct FocusTracker {
    items: Vec<Vec<bool>>,
    current: (u64, u64),
}

pub struct FocusedRes {
    pub old: (u64, u64),
    pub new: (u64, u64),
}

impl FocusedRes {
    pub fn new(old: (u64, u64), new: (u64, u64)) -> Self {
        FocusedRes { 
            old,
            new
        }
    }
    pub fn is_same(&self) -> bool {
        self.old.0 == self.new.0 && self.old.1 == self.new.1 
    }
}
impl FocusTracker {
    pub fn new(len: Vec<u64>) -> Self {
        let mut items = Vec::<Vec<bool>>::new();
        for col in len {
            let mut row = Vec::<bool>::new();
            for _ in 0..col {
                row.push(false);
            }
            items.push(row);
        }
        if let Some(first) = items.first_mut() {
            if let Some(first) = first.first_mut() {
                *first = true;
            }
        };
        Self {
            items,
            current: (0, 0)
        }
    }

    pub fn change_focused(&mut self, axis: Axis) -> FocusedRes {
        let old = self.current;
        match axis {
            Axis::Empty => return FocusedRes::new(old, old),
            Axis::Horizontal(axis) => {
                match axis {
                    Horizontal::Left => {
                        self.current.1 = self.current.1.saturating_sub(1);
                    },
                    Horizontal::Right => {
                        self.current.1 = self.current.1.saturating_add(1);
                        if self.items[self.current.0 as usize].len()  as u64 >= self.current.1 {
                            self.current.1 = self.items[self.current.0 as usize].len().saturating_sub(1) as u64;
                        }
                    }
                }
            }
            Axis::Vertical(axis) => {
                match axis {
                    Vertical::Down => {
                        self.current.0 = self.current.0.saturating_sub(1);
                        
                    }
                    Vertical::Up => {
                        self.current.0 = self.current.0.saturating_add(1);
                        if self.current.0 >= self.items.len() as u64 {
                            self.current.0 = self.items.len().saturating_sub(1) as u64;
                        }
                    }
                }
            }
        };
        FocusedRes::new(old, self.current)
    }
    pub fn current(&self) -> (u64, u64) {
        self.current
    }
}