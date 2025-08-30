//! Layout directions

/// Alignment across the layout direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

/// Layout directions for content
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Vertical(Align),
    Horizontal(Align),
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Vertical(Align::default())
    }
}

impl Direction {
    pub fn align(&self) -> Align {
        match self {
            Direction::Horizontal(align) => *align,
            Direction::Vertical(align) => *align,
        }
    }
}
