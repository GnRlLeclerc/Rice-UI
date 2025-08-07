//! Gap between children in a layout

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Gap {
    /// Fixed pixel gap
    Fixed(usize),
    /// Same gap between all children to fill parent
    Auto,
}

impl Default for Gap {
    fn default() -> Self {
        Gap::Fixed(0)
    }
}
